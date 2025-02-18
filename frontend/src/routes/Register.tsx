import { useNavigate } from "react-router-dom";
import { useState } from "react";

function Register() {
  const [response, setResponse] = useState("");
  const navigate = useNavigate();

  function handleSubmit(event: React.SyntheticEvent<HTMLFormElement>) {
    event.preventDefault();
    const formData = new FormData(event.currentTarget);
    const data = Object.fromEntries(formData);

    const password = data["password"];
    const repeatedPassword = data["repeatPassword"];

    const email = data["email"] as string;
    const emailRegex = /^[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Z]{2,4}$/i;

    if (!emailRegex.test(email)) {
      setResponse("Email is not valid");
      console.log("invalid email");
      return;
    }

    if (password !== repeatedPassword) {
      setResponse("Passwords do not match");
      return;
    }

    fetch("/api/auth/register", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(data),
    })
      .then(async (data) => {
        return { status: data.status, body: await data.text() };
      })
      .then((data) => {
        console.log(data);
        if (data.status == 400 && data.body == "Username") {
          setResponse("Username already taken");
        } else if (data.status == 400 && data.body == "Email") {
          setResponse("Email already taken");
        } else if (data.status == 201) {
          setResponse("");
          navigate("/Login", { replace: true });
        }
      });
  }

  console.log(response);

  return (
    <div>
      <form id="registerForm" className="accountForms" onSubmit={handleSubmit}>
        <p>Register</p>
        <input
          className="accountInputs"
          type="text"
          name="username"
          placeholder="Username"
        />
        <input
          className="accountInputs"
          type="email"
          name="email"
          placeholder="Email"
        />
        <input
          className="accountInputs"
          type="password"
          name="password"
          placeholder="Password"
        />
        <input
          className="accountInputs"
          type="password"
          name="repeatPassword"
          placeholder="Repeat password"
        />
        <button className="accountBtns" type="submit">
          Register
        </button>
      </form>
    </div>
  );
}

export default Register;
