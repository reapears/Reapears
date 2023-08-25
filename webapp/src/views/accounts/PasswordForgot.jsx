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

export function PasswordForgot(props) {
  const styles = useStyles();

  const [user, setUser] = useState({
    email: "",
  });

  const onChange = (event) => {
    const key = event.target.name;
    const value = event.target.value;
    setUser((oldUser) => ({ ...oldUser, [key]: value }));
  };

  const submitForm = (event) => {
    forgotPassword(user);
    event.preventDefault();
  };

  return (
    <form className={styles.root} onSubmit={submitForm}>
      <Field label="Your email address" {...props}>
        <Input
          name="email"
          value={user.email}
          onChange={onChange}
          type="email"
        />
      </Field>

      <Button appearance="primary" {...props}>
        Forgot password
      </Button>

      <pre>{JSON.stringify(user, true, 2)}</pre>
    </form>
  );
}

function forgotPassword(user) {
  console.log(JSON.stringify(user));
}
