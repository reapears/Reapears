import React from "react";
import reapersLogo from "../assets/reapears-logo.png";

import {
  Link,
  Text,
  makeStyles,
  Menu,
  MenuTrigger,
  MenuList,
  MenuPopover,
  Divider,
  MenuItemLink,
} from "@fluentui/react-components";
import { Navigation16Regular, Person16Filled } from "@fluentui/react-icons";

const useStyles = makeStyles({
  menuPersonIcon: {
    color: "#fff",
  },
  menuIcon: {
    color: "#222",
  },
  navbarCta: {
    color: "#222",
  },
});

export function Header(props) {
  const styles = useStyles();

  return (
    <>
      <header className="header">
        <Link href="/">
          <img className="logo" src={reapersLogo} alt="" />
        </Link>

        <nav className="header-navbar">
          <div className="navbar-cta">
            <Link href="" {...props} className={styles.navbarCta}>
              <Text size={400}>Become a farmer</Text>
            </Link>
          </div>

          <Menu>
            <MenuTrigger>
              <div className="navbar-menu">
                <div className="navbar-menu-line">
                  <Navigation16Regular className={styles.menuIcon} />
                </div>
                <div className="navbar-menu-persona">
                  <Person16Filled className={styles.menuPersonIcon} />
                </div>
              </div>
            </MenuTrigger>
            <MenuPopover>
              <MenuList>
                <MenuItemLink>Sign Up</MenuItemLink>
                <Divider />
                <MenuItemLink>Log In</MenuItemLink>
              </MenuList>
            </MenuPopover>
          </Menu>
        </nav>
      </header>
    </>
  );
}
