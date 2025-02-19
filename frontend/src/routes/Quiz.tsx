import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";

function Quiz() {
  const [selectedBtn, setSelectedBtn] = useState<string | null>(null);
  const navigate = useNavigate();
  const handleBtnClick = (event: React.MouseEvent<HTMLButtonElement>) => {
    event.preventDefault();
    const btnText = event.currentTarget.textContent;
    if (selectedBtn === btnText) {
      setSelectedBtn(null);
    } else {
      setSelectedBtn(btnText);
    }
  };

  const handleFormSubmit = (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();
  };

  useEffect(() => {
    console.log("Selected button:", selectedBtn);
  }, [selectedBtn]);

  useEffect(() => {
    console.log("page loaded");
    const gameId = sessionStorage.getItem("gameId");
    console.log(gameId);
    if (gameId) {
      fetch("api/game/question", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ game_id: gameId }),
      }).then((data) => console.log(data));
    } else {
      navigate("/", { replace: true });
    }
  }, []);

  return (
    <div id="quizContainer">
      <form id="quiz" onSubmit={handleFormSubmit}>
        <span id="question">Silli silli silli</span>
        <div className="quizBtnContainer">
          {["a", "b", "c", "d"].map((num) => (
            <button
              key={num}
              className="quizBtn"
              onClick={handleBtnClick}
              style={{
                border:
                  selectedBtn === num ? "2px solid var(--rich-black)" : "none",
              }}
            >
              {num}
            </button>
          ))}
        </div>
        <button className="submitBtn">Next</button>
      </form>
    </div>
  );
}

export default Quiz;
