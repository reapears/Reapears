import { React, useState } from "react";

import {
  Field,
  Input,
  Button,
  makeStyles,
  shorthands,
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

export function AccountSignUp(props) {
  const styles = useStyles();
  const [user, setUser] = useState({
    firstName: "",
    lastName: "",
    email: "",
    password: "",
  });

  const onChange = (event) => {
    const key = event.target.name;
    const value = event.target.value;
    setUser((oldUser) => ({ ...oldUser, [key]: value }));
  };

  const submitForm = (event) => {
    signUpUser(user);
    event.preventDefault();
  };
  return (
    <form className={styles.root} onSubmit={submitForm}>
      <Field label="First name" required {...props}>
        <Input name="firstName" value={user.firstName} onChange={onChange} />
      </Field>

      <Field label="Last name" {...props}>
        <Input name="lastName" value={user.lastName} onChange={onChange} />
      </Field>

      <Field label="Email" {...props}>
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
        Sign Up
      </Button>

      <div style={{ display: "flex", gap: "20px" }}>
        <Link href="https://www.bing.com" {...props}>
          Login
        </Link>
        <Link href="https://www.bing.com" {...props}>
          Forgot password?
        </Link>
      </div>

      <pre>{JSON.stringify(user, true, 2)}</pre>
    </form>
  );
}

function signUpUser(user) {
  console.log(JSON.stringify(user));
}
