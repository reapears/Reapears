import { React, useState } from "react";

export function EmailChange() {
  const [info, setInfo] = useState({
    email: "",
  });

  const onChange = (event) => {
    const key = event.target.name;
    const value = event.target.value;
    setInfo((oldInfo) => ({ ...oldInfo, [key]: value }));
  };

  const submitForm = (event) => {
    changeEmail(info);
    event.preventDefault();
  };

  return (
    <form style={{ display: "flex", flexDirection: "column" }}>
      <div>
        <label htmlFor="account-email">Your email address:</label>
        <input
          value={info.email}
          id="account-email"
          name="email"
          onChange={onChange}
          type="email"
          required
        />
      </div>

      <button onClick={submitForm}>change email</button>
      <pre>{JSON.stringify(info, true, 2)}</pre>
    </form>
  );
}

function changeEmail(info) {
  console.log(JSON.stringify(info));
}
