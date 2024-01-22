import { useState, useMemo, useCallback } from "react";
import { Chessboard } from "react-chessboard";
import { Chess } from "chess.js";
import CustomDialog from "./CustomDialog.js";
import axios from "axios";

function Game({ players, room, orientation, cleanup }) {
  const chess = useMemo(() => new Chess(), []); // <- 1
  const [fen, setFen] = useState(chess.fen()); // <- 2
  const [over, setOver] = useState("");

  const makeAMove = useCallback(
    (move) => {
      try {
        const result = chess.move(move); // update Chess instance
        setFen(chess.fen()); // update fen state to trigger a re-render

        console.log("over, checkmate", chess.isGameOver(), chess.isCheckmate());

        if (chess.isGameOver()) { // check if move led to "game over"
          if (chess.isCheckmate()) { // if reason for game over is a checkmate
            // Set message to checkmate. 
            setOver(
              `Checkmate! ${chess.turn() === "w" ? "black" : "white"} wins!`
            ); 
            // The winner is determined by checking which side made the last move
          } else if (chess.isDraw()) { // if it is a draw
            setOver("Draw"); // set message to "Draw"
          } else {
            setOver("Game over");
          }
        }

        return result;
      } catch (e) {
        return null;
      } // null if the move was illegal, the move object if the move was legal
    },
    [chess]
  );

  async function getEngineMove(wtime, btime, winc, binc) {
    const request_fen = chess.fen().split(" ").join("+").split("/").join("%2F");
    const params = new URLSearchParams();
    params.append("fen", request_fen);
    params.append("wtime", wtime);
    params.append("btime", btime);
    params.append("winc", winc);
    params.append("binc", binc);
    const response = await axios.get("http://127.0.0.1:8080/search/", params, {
      headers: {
        "Access-Control-Allow-Origin": "*",
        "Access-Control-Allow-Headers": "Origin, X-Requested-With, Content-Type, Accept",
        "Content-Type": "x-www-urlencoded",
      }
    });
    const bestmove = response.split("\n")[2].split(" ")[1];
    chess.move(bestmove);
  }

  // onDrop function
  function onDrop(sourceSquare, targetSquare) {
    const moveData = {
      from: sourceSquare,
      to: targetSquare,
      color: chess.turn(),
      // promotion: "q",
    };

    const move = makeAMove(moveData);

    // illegal move
    if (move === null) return false;

    getEngineMove(600, 600, 0, 0).then(
      function(value) { chess.move(value); }
    );

    return true;
  }
  
  // Game component returned jsx
  return (
    <>
      <div className="board">
        <Chessboard position={fen} onPieceDrop={onDrop} />  {/**  <- 4 */}
      </div>
      <CustomDialog // <- 5
        open={Boolean(over)}
        title={over}
        contentText={over}
        handleContinue={() => {
          setOver("");
        }}
      />
    </>
  );
}

export default Game;
