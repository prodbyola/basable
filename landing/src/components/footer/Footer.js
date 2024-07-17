import React from "react";
import "./Footer.css";
import logo from "./images/logo.svg";
import linkedin from "./images/LinkedIn - Original.svg";
import twitter from "./images/prime_twitter.svg";
import github from "./images/Github - Original.svg";

function Footer() {
  return (
    <div>
      <footer>
        <div className="main-footer-container">
          <div className="footer-top-container">
            <div className="footer-left-side">
              <img className="footer-logo" src={logo} alt="logo" />
              <p className="footer-left-text">
                Open-source data management, visualization, and business
                insights for everyone.
              </p>
            </div>
            <div className="footer-right-side">
              <div className="footer-right-tabs1">
                <h2>Our Services</h2>
                <div className="">
                  <ul className="footer-listed-items footer-listed-items-text-sm1">
                    <li>Help Center</li>
                    <li>FAQs</li>
                    <li>Email</li>
                  </ul>
                </div>
              </div>

              <div className="footer-right-tabs2">
                <h2>Company</h2>
                <div className="">
                  <ul className="footer-listed-items footer-listed-items-text-sm2">
                    <li>About us</li>
                    <li>Career</li>
                    <li>Management</li>
                    <li>Blog</li>
                  </ul>
                </div>
              </div>

              <div className="footer-right-tabs3">
                <h2>Follow Us</h2>
                <div className="">
                  <ul className="footer-listed-items2 footer-listed-items-text-sm3">
                    <a href="#">
                      <img
                        className="social-icon"
                        src={linkedin}
                        alt="linkedin"
                      />
                    </a>
                    <a href="#">
                      <img
                        className="social-icon"
                        src={twitter}
                        alt="linkedin"
                      />
                    </a>
                    <a href="#">
                      <img
                        className="social-icon"
                        src={github}
                        alt="linkedin"
                      />
                    </a>
                  </ul>
                </div>
              </div>
            </div>
          </div>
          <div className="footer-bottom-container">
            <div className="bottom-footer-inner">
              {" "}
              <div className="footer-bottom-left">
                <div className="copyright">
                  <span className="copyright-copy">&#169;</span>
                </div>
                <p className="footer-bottom-text">2024 BASABLE</p>
              </div>
              <p className="footer-bottom-text">All Rights Reserved</p>
            </div>
          </div>
        </div>
      </footer>
    </div>
  );
}

export default Footer;
