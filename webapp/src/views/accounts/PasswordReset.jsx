import { React, useState } from "react";

import {
  Field,
  Input,
  shorthands,
  makeStyles,
  Button,
} from "@fluentui/react-components";

const useStyles = makeStyles({
  root: {
    display: "flex",
    flexDirection: "column",
    ...shorthands.gap("20px"),
    maxWidth: "400px",
  },
});

export function PasswordReset(props) {
  const styles = useStyles();
  const [password, setPassword] = useState({
    new: "",
    confirm: "",
  });

  const onChange = (event) => {
    const key = event.target.name;
    const value = event.target.value;
    setPassword((oldPassword) => ({ ...oldPassword, [key]: value }));
  };

  const submitForm = (event) => {
    resetPassword(password);
    event.preventDefault();
  };

  return (
    <form className={styles.root} onSubmit={submitForm}>
      <Field label="New password" {...props}>
        <Input
          name="new"
          value={password.new}
          onChange={onChange}
          type="password"
          required
        />
      </Field>

      <Field label="Confirm password" {...props}>
        <Input
          name="confirm"
          value={password.confirm}
          onChange={onChange}
          type="password"
          required
        />
      </Field>

      <Button appearance="primary" {...props}>
        Rest password
      </Button>

      <pre>{JSON.stringify(password, true, 2)}</pre>
    </form>
  );
}

function resetPassword(password) {
  console.log(JSON.stringify(password));
}
