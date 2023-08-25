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

export function PasswordChange(props) {
  const styles = useStyles();

  const [password, setPassword] = useState({
    current: "",
    new: "",
    confirm: "",
  });

  const onChange = (event) => {
    const key = event.target.name;
    const value = event.target.value;
    setPassword((oldPassword) => ({ ...oldPassword, [key]: value }));
  };

  const submitForm = (event) => {
    changePassword(password);
    event.preventDefault();
  };

  return (
    <form className={styles.root} onSubmit={submitForm}>
      <Field label="Current password" {...props}>
        <Input
          name="current"
          value={password.current}
          onChange={onChange}
          type="password"
          required
        />
      </Field>

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
        Change password
      </Button>

      <pre>{JSON.stringify(password, true, 2)}</pre>
    </form>
  );
}

function changePassword(password) {
  console.log(JSON.stringify(password));
}
