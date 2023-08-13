import { React, useState } from "react";

export function EmailChangeApprove() {
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
        <label htmlFor="approve-code">Email change approval code:</label>
        <input
          value={info.code}
          id="approve-code"
          name="code"
          onChange={onChange}
          type="text"
          required
        />
      </div>

      <button onClick={submitForm}>approve email change</button>
      <pre>{JSON.stringify(info, true, 2)}</pre>
    </form>
  );
}

function approveEmailChange(info) {
  console.log(JSON.stringify(info));
}
