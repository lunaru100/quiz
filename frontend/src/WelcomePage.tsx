import { useNavigate } from "react-router-dom";

function WelcomePage() {
  const navigate = useNavigate();
  const handleBtnClick = (event: React.MouseEvent<HTMLButtonElement>) => {
    const btnText = event.currentTarget.textContent;
    const numQuestions = btnText ? parseInt(btnText) : 0;

    console.log("Selected number of questions:", numQuestions);
    navigate(`/quiz`);
  };

  return (
    <div id="welcomePage">
      <span id="welcome">Welcome to our Arcane + JJK quiz</span>
      <span id="selectQuestions">Select the number of questions</span>
      <div id="btnContainer">
        <button className="questionButton" onClick={handleBtnClick}>
          5
        </button>
        <button className="questionButton" onClick={handleBtnClick}>
          10
        </button>
        <button className="questionButton" onClick={handleBtnClick}>
          15
        </button>
        <button className="questionButton" onClick={handleBtnClick}>
          20
        </button>
      </div>
    </div>
  );
}

export default WelcomePage;
