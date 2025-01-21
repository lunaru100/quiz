import { useEffect, useState } from "react";

function Quiz() {
  const [selectedBtn, setSelectedBtn] = useState<string | null>(null);
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
