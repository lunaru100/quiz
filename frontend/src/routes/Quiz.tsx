import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";

interface Question {
  question: string;
  answers: string[];
  category: string;
  id: string;
}

function Quiz() {
  const [selectedBtn, setSelectedBtn] = useState<string | null>(null);
  const navigate = useNavigate();
  const [question, setQuestion] = useState<Question>();
  const handleBtnClick = (
    event: React.MouseEvent<HTMLButtonElement>,
    answerId: number
  ) => {
    event.preventDefault();
    const btnText = event.currentTarget.textContent;
    console.log(selectedBtn);
    if (selectedBtn === btnText) {
      setSelectedBtn(null);
      sessionStorage.removeItem("answerId");
    } else {
      setSelectedBtn(btnText);
      sessionStorage.setItem("answerId", answerId.toString());
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
    sessionStorage.removeItem("answerId");
    if (gameId) {
      fetch("api/game/question", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ game_id: gameId }),
      })
        .then((data) => {
          return data.json();
        })
        .then((data) => {
          setQuestion({
            question: data.text,
            answers: data.answers,
            category: data.category,
            id: data.id,
          });
        });
    } else {
      navigate("/", { replace: true });
    }
  }, []);

  const handleSubmit = (event: React.MouseEvent<HTMLButtonElement>) => {
    event.preventDefault();
    if (sessionStorage.getItem("answerId") != null) {
      console.log(sessionStorage.getItem("answerId")?.valueOf());
      fetch("api/game/answer", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          answer: parseInt(sessionStorage.getItem("answerId")?.valueOf() || ""),
          game_id: sessionStorage.getItem("gameId"),
        }),
      })
        .then((data) => {
          return data.json();
        })
        .then((data) => {
          if (data) {
            const score = parseInt(sessionStorage.getItem("score") || "") + 1;
            sessionStorage.setItem("score", score.toString());
          } else {
            console.log("wrong");
          }
          console.log(
            "question count: " + sessionStorage.getItem("currentQuestionCount")
          );
          const questionCount =
            parseInt(sessionStorage.getItem("currentQuestionCount") || "") + 1;
          sessionStorage.setItem(
            "currentQuestionCount",
            questionCount.toString()
          );
          const count = parseInt(sessionStorage.getItem("questionCount") || "");
          if (count == questionCount) {
            navigate("/results", { replace: true });
          }
          window.location.reload();
        });
    }
    //navigate("/quiz", { replace: true });
  };

  if (!question) {
    return <div></div>;
  }

  return (
    <div id="quizContainer">
      <form id="quiz" onSubmit={handleFormSubmit}>
        <span id="question">{question.question}</span>
        <div className="quizBtnContainer">
          {[
            // question.answers[0],
            // question.answers[1],
            // question.answers[2],
            // question.answers[3],
            0, 1, 2, 3,
          ].map((num) => (
            <button
              key={question.answers[num]}
              className="quizBtn"
              onClick={(event) => handleBtnClick(event, num)}
              style={{
                border:
                  selectedBtn === question.answers[num]
                    ? "2px solid var(--rich-black)"
                    : "none",
              }}
            >
              {question.answers[num]}
            </button>
          ))}
        </div>
        <button className="submitBtn" onClick={handleSubmit}>
          Next
        </button>
      </form>
    </div>
  );
}

export default Quiz;
