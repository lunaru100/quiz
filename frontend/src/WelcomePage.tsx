import { useState } from "react";
import { useNavigate } from "react-router-dom";

function WelcomePage() {
  const navigate = useNavigate();
  const [numQuestions, setNumQuestions] = useState(0);
  const [category, setCategory] = useState("");
  const [selectedNumButton, setSelectedNumButton] = useState<string | null>(
    null
  );
  const [selectedCategoryButton, setSelectedCategoryButton] = useState<
    string | null
  >(null);

  const handleNumBtnClick = (event: React.MouseEvent<HTMLButtonElement>) => {
    event.preventDefault();
    const btnText = event.currentTarget.textContent;
    if (selectedNumButton === btnText) {
      setSelectedNumButton(null);
      setNumQuestions(0);
    } else {
      setSelectedNumButton(btnText);
      setNumQuestions(btnText ? parseInt(btnText) : 0);
    }
  };

  const handleCatBtnClick = (event: React.MouseEvent<HTMLButtonElement>) => {
    event.preventDefault();
    const btnText = event.currentTarget.textContent;
    if (selectedCategoryButton === btnText) {
      setSelectedCategoryButton(null);
      setCategory("");
    } else {
      setSelectedCategoryButton(btnText);
      setCategory(btnText ? btnText : "");
    }
  };

  const handleSubmit = (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    console.log("Number of questions:", numQuestions);
    console.log("Category:", category);
    navigate(`/quiz`);
  };

  return (
    <div id="welcomePage">
      <span id="welcome">Welcome to our Arcane + JJK quiz</span>
      <form id="startQuizForm" onSubmit={handleSubmit}>
        <span className="selectQuestions">Select the number of questions</span>
        <div id="btnContainer">
          {["5", "10", "15", "20"].map((num) => (
            <button
              key={num}
              className="questionButton"
              onClick={handleNumBtnClick}
              style={{
                backgroundColor:
                  selectedNumButton === num ? "#77aca2ff" : "#468189ff",
              }}
            >
              {num}
            </button>
          ))}
        </div>
        <span className="selectQuestions" style={{ marginTop: "4vh" }}>
          Select category
        </span>
        <div id="categoryBtnContainer">
          {["All", "Arcane", "JJK"].map((cat) => (
            <button
              key={cat}
              className="questionButton"
              onClick={handleCatBtnClick}
              style={{
                backgroundColor:
                  selectedCategoryButton === cat ? "#77aca2ff" : "#468189ff",
              }}
            >
              {cat}
            </button>
          ))}
        </div>
        <button id="startBtn" type="submit">
          Start Quiz
        </button>
      </form>
    </div>
  );
}

export default WelcomePage;
