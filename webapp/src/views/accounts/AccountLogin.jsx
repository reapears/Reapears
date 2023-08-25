import { React, useState } from "react";

import {
  Field,
  Input,
  makeStyles,
  shorthands,
  Button,
  Link,
} from "@fluentui/react-components";

const useStyles = makeStyles({
  root: {
    display: "flex",
    flexDirection: "column",
    ...shorthands.gap("20px"),
    maxWidth: "400px",
  },
});

export function AccountLogin(props) {
  const styles = useStyles();

  const [user, setUser] = useState({
    email: "",
    password: "",
  });

  const onChange = (event) => {
    const key = event.target.name;
    const value = event.target.value;
    setUser((oldUser) => ({ ...oldUser, [key]: value }));
  };

  const submitForm = (event) => {
    loginUser(user);
    event.preventDefault();
  };

  return (
    <form className={styles.root} onSubmit={submitForm}>
      <Field label="Email" required {...props}>
        <Input
          name="email"
          value={user.email}
          onChange={onChange}
          type="email"
        />
      </Field>

      <Field label="Password" required {...props}>
        <Input
          name="password"
          value={user.password}
          onChange={onChange}
          type="password"
        />
      </Field>

      <Button appearance="primary" {...props}>
        Login
      </Button>

      <div style={{ display: "flex", gap: "20px" }}>
        <Link href="https://www.bing.com" {...props}>
          Sign Up
        </Link>
        <Link href="https://www.bing.com" {...props}>
          Forgot password?
        </Link>
      </div>
      <pre>{JSON.stringify(user, true, 2)}</pre>
    </form>
  );
}

function loginUser(user) {
  console.log(JSON.stringify(user));
}
