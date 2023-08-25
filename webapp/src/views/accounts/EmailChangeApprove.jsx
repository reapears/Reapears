import { React, useState } from "react";

import {
  Field,
  Input,
  shorthands,
  makeStyles,
Button
} from "@fluentui/react-components";

const useStyles = makeStyles({
  root: {
    display: "flex",
    flexDirection: "column",
    ...shorthands.gap("20px"),
    maxWidth: "400px",
  },
});

export function EmailChangeApprove(props) {
  const styles = useStyles();

  const [user, setUser] = useState({
    code: "",
  });

  const onChange = (event) => {
    const key = event.target.name;
    const value = event.target.value;
    setUser((oldUser) => ({ ...oldUser, [key]: value }));
  };

  const submitForm = (event) => {
    approveEmailChange(user);
    event.preventDefault();
  };

  return (
    <form className={styles.root} onSubmit={submitForm}>
      <Field label="Email change approval code" {...props}>
        <Input name="code" value={user.code} onChange={onChange} />
      </Field>

      <Button appearance="primary" {...props}>
        approve email change
      </Button>

      <pre>{JSON.stringify(user, true, 2)}</pre>
    </form>
  );
}

function approveEmailChange(user) {
  console.log(JSON.stringify(user));
}
