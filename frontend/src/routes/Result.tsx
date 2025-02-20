import { useNavigate } from "react-router-dom";

function Result() {
  const navigate = useNavigate();
  return (
    <div id="result">
      <p>Thank you for playing our quiz</p>
      <p>Your score: {sessionStorage.getItem("score")}</p>
      <button
        className="startBtn"
        onClick={() => navigate("/", { replace: true })}
      >
        Play again
      </button>
    </div>
  );
}

export default Result;
