import React from "react";
import "./Footer.css";
import logo from "../images/logo.svg";
import linkedin from "../images/LinkedIn - Original.svg";
import twitter from "../images/prime_twitter.svg";
import github from "../images/Github - Original.svg";

function FooterSection() {
  return (
    <div>
      <footer>
        <div className="footer-section">
          <div className="top-footer">
            <div className="left-side">
              <img className="logo" src={logo} alt="logo" />
              <p className="left-text">
                Open-source data management, visualization, and business
                insights for everyone.
              </p>
            </div>

            <div className="right-side">
              <div className="right-mid">
                <h5 className="right-mid-head">Our Services</h5>
                <div className="dropdown">
                  <a href="">Help Center</a>
                  <a href="">FAQ</a>
                  <a href="">Email</a>
                </div>
              </div>

              <div className="right-mid">
                <h5 className="right-mid-head">Company</h5>
                <div className="dropdown">
                  <a href="">About us</a>
                  <a href="">Career</a>
                  <a href="">Management</a>
                  <a href="">Blog</a>
                </div>
              </div>

              <div className="right-mid">
                <h5 className="right-mid-head">Our Services</h5>
                <div className="dropdown dropdown-row">
                  <a href="#">
                    <img
                      className="social-icon"
                      src={linkedin}
                      alt="linkedin"
                    />
                  </a>
                  <a href="#">
                    <img className="social-icon" src={twitter} alt="twitter" />
                  </a>
                  <a href="#">
                    <img className="social-icon" src={github} alt="github" />
                  </a>
                </div>
              </div>
            </div>
          </div>

          <div className="bottom-footer">
            <div className="bottom-right">
              <p className="copryright">&copy;</p>
              <p className="bottom-text">2024 BASABLE</p>
            </div>
            <p className="bottom-text">All Rights Reserved</p>
          </div>
        </div>
      </footer>
    </div>
  );
}

export default FooterSection;
