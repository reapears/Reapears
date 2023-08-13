import { React, useState } from "react";

export function EmailChangeConfirm() {
  const [info, setInfo] = useState({
    code: "",
  });

  const onChange = (event) => {
    const key = event.target.name;
    const value = event.target.value;
    setInfo((oldInfo) => ({ ...oldInfo, [key]: value }));
  };

  const submitForm = (event) => {
    approveEmailChange(info);
    event.preventDefault();
  };

  return (
    <form style={{ display: "flex", flexDirection: "column" }}>
      <div>
        <label htmlFor="confirm-code">Email confirm code:</label>
        <input
          value={info.code}
          id="confirm-code"
          name="code"
          onChange={onChange}
          type="text"
          required
        />
      </div>

      <button onClick={submitForm}>confirm email change</button>
      <pre>{JSON.stringify(info, true, 2)}</pre>
    </form>
  );
}

function approveEmailChange(info) {
  console.log(JSON.stringify(info));
}
