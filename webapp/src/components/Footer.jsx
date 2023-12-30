import React from "react";
import reapersLogo from "../assets/reapears-logo.png";
import { Link, makeStyles, Subtitle2, Text } from "@fluentui/react-components";

const useStyles = makeStyles({
  footerLink: {
    fontSize: "1.6rem",
    color: "#777",
  },

  footerAddress: {
    fontSize: "1.6rem",
    color: "#555",
  },
});

export function Footer() {
  const styles = useStyles();
  const currentYear = new Date().getFullYear();
  return (
    <footer className="footer">
      <div className="footer-container grid--footer">
        <div className="logo-col">
          <FooterLogo />

          {/* <ul className="social-links">
            <li>
              <a href="" className="footer-link">
                <ion-icon
                  className="social-icon"
                  name="logo-twitter"
                ></ion-icon>
              </a>
            </li>
          </ul> */}

          <p className="copyright">
            Copyright &copy; <span className="current-year">{currentYear}</span>{" "}
            by Reapears, Inc. All right reserved.
          </p>
        </div>
        <div className="address-col">
          <FooterHeader heading={"Contact US"} />
          <address className="contacts">
            <div className="footer-address">
              <Text className={styles.footerAddress} size={400}>
                Address: P.O.Box 100 Outapi, UP 16001
              </Text>
            </div>

            <Text className={styles.footerAddress} size={400}>
              reapears@outlook.com
            </Text>

            <Text className={styles.footerAddress} size={400}>
              +264814445973
            </Text>

            {/* <FooterLink link={"tel:0814445973"} name={"+264814445973"} />
            <FooterLink
              link={"mailto:reapears@outlook.com"}
              name={" reapears@outlook.com"}
            /> */}
          </address>
        </div>

        <nav className="nav-col">
          <FooterHeader heading={"Account"} />
          <ul className="footer-nav">
            <li>
              <FooterLink name={"Create account"} />
            </li>
            <li>
              <FooterLink name={"Sign in"} />
            </li>
            {/* <li>
              <FooterLink name={"iOS app"} />
            </li>
            <li>
              <FooterLink name={"Android app"} />
            </li> */}
          </ul>
        </nav>

        <nav className="nav-col">
          <FooterHeader heading={"Company"} />
          <ul className="footer-nav">
            <li>
              <FooterLink name={"About Reapers"} />
            </li>
            <li>
              <FooterLink name={"For Business"} />
            </li>
            <li>
              <FooterLink name={"Farmers"} />
            </li>
            <li>
              <FooterLink name={"Careers"} />
            </li>
          </ul>
        </nav>

        <nav className="nav-col">
          <FooterHeader heading={"Resources"} />
          <ul className="footer-nav">
            <li>
              <FooterLink name={"Cultivars"} />
            </li>
            <li>
              <FooterLink name={" Help center"} />
            </li>
            <li>
              <FooterLink name={"Privacy & terms"} />
            </li>
          </ul>
        </nav>
      </div>
    </footer>
  );
}

function FooterLogo() {
  return (
    <div className="footer-logo">
      <Link>
        <img src={reapersLogo} className="logo" alt="" />
      </Link>
    </div>
  );
}

function FooterLink({ link, name }) {
  const styles = useStyles();
  link = link ? link : "";
  return (
    <Link href={link} className={styles.footerLink}>
      <Text size={400}>{name}</Text>
    </Link>
  );
}

function FooterHeader({ heading }) {
  return (
    <div className="footer-heading">
      <Subtitle2>{heading}</Subtitle2>
    </div>
  );
}
