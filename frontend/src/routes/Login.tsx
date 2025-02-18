import { useNavigate } from "react-router-dom";
import { useState } from "react";

function Login() {
  const [response, setResponse] = useState("");
  const navigate = useNavigate();

  function handleSubmit(event: React.SyntheticEvent<HTMLFormElement>) {
    event.preventDefault();
    const formData = new FormData(event.currentTarget);
    const data = Object.fromEntries(formData);
    fetch("/api/auth/login", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(data),
    }).then((data) => {
      console.log(data);
      if (data.status === 401) {
        setResponse("Incorrect login or password");
      } else if (data.status === 500) {
        setResponse("Internal server error");
      } else if (data.status === 200) {
        setResponse("");
        data.json().then((data) => {
          console.log(data);
          sessionStorage.setItem("username", data.username);
          sessionStorage.setItem("userId", data.id);
        });
        navigate("/", { replace: true });
        window.location.reload();
      }
    });
  }

  console.log(response);

  return (
    <div id="login">
      <form id="loginForm" className="accountForms" onSubmit={handleSubmit}>
        <p>Log in</p>
        <input
          className="accountInputs"
          type="text"
          name="username"
          placeholder="Username"
        />
        <input
          className="accountInputs"
          type="password"
          name="password"
          placeholder="Password"
        />
        <button className="accountBtns" type="submit">
          Log in
        </button>
      </form>
    </div>
  );
}

export default Login;
