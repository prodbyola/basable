import React, { useState } from "react";
import "./Contact.css";
import icon from "../images/icon.svg";
import caller from "../images/contactpic.svg";

function Contact() {
  const [name, setName] = useState("");
  const [email, setEmail] = useState("");
  const [message, setMessage] = useState('');

  const handleSubmit = (event) => {
    event.preventDefault();
  };

  return (
    <div className="container">
      <section className="landing-contact">
        <div className="contact-left">
          <div className="contact-mid-container">
            {" "}
            <div className="contact-head">
              {" "}
              <div className="contact-get-started">
                <img className="logo-icon" src={icon} alt="icon" />
                <p className="get-started-text">Get Started</p>
              </div>
              <div className="contact-header-text">
                <h1 className="contact-h1">Contact Us</h1>
                <p className="form-message-text">
                  Please fill this form to send us any questions or concerns you
                  have.
                </p>
              </div>
            </div>
            <form className="contact-form" onSubmit={handleSubmit}>
              <div className="form-field">
                {" "}
                {/* <div className="contact-fullname"> */}
                <div className="form-label">
                  <p className="label-text">Full Name</p>
                  <input
                    className="input-field"
                    type="text"
                    value={name}
                    onChange={(e) => setName(e.target.value)}
                  ></input>
                </div>
                {/* </div> */}
                {/* <div className="contact-fullname"> */}
                <div className="form-label">
                  <p className="label-text2">Email</p>
                  <input
                    className="input-field"
                    type="text"
                    value={name}
                    onChange={(e) => setName(e.target.value)}
                  ></input>
                </div>
                {/* </div> */}
                <div className="">
                  <div className="message-container">
                    <p className="label-text3">Your Message</p>
                    <input
                      className="message-field"
                      type="text"
                      value={name}
                      onChange={(e) => setName(e.target.value)}
                    ></input>
                  </div>
                </div>
              </div>
              {/* <div className="btn-container"> */}
                <button className="contact-form-button">Submit</button>
              {/* </div> */}
            </form>
          </div>
        </div>
        <div className="contact-image-container">
          <img
            className="contact-caller"
            src={caller}
            alt="caller"
            width="708px"
            height="899px"
          />
        </div>
      </section>
    </div>
  );
}

export default Contact;
