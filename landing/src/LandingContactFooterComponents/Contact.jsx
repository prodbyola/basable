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
          <div className="contact-get-started">
            <img className="logo-icon" src={icon} alt="icon" />
            <p>Get Started</p>
          </div>
          <div className="contact-header-text">
            <h1 className="contact-h1">Contact Us</h1>
            <p className="form-message-text">
              Please fill this form to send us any questions or concerns you
              have.
            </p>
          </div>
          <form className="contact-form" onSubmit={handleSubmit}>
            <div className="contact-fullname">
              <label className="form-label">
                Full Name
                <input
                  type="text"
                  value={name}
                  onChange={(e) => setName(e.target.value)}
                ></input>
              </label>
            </div>

            <div className="contact-email">
              <label className="form-label">
                Email
                <input
                  type="email"
                  value={email}
                  onChange={(e) => setEmail(e.target.value)}
                ></input>
              </label>
            </div>

            <div className="contact-messsage">
              <label className="form-label">
                Your Message
                <textarea
                  value={message}
                  onChange={(e) => setMessage(e.target.value)}
                ></textarea>
              </label>
            </div>
            <button className="contact-form-button">Submit</button>
          </form>
        </div>
        <div className="contact-image-container">
          <img className="contact-caller" src={caller} alt="caller" width="708px" height="899px"/>
        </div>
      </section>
    </div>
  );
}

export default Contact;
