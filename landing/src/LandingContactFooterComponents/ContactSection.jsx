import React, { useState } from "react";
import "./Contact.css";
import icon from "../images/icon.svg";
import contacthero from "../images/contactpic.svg";


function ContactSection() {
  const [formData, setFormData] = useState({
    fullname: " ",
    email: " ",
    message: " ",
  });

  const handleChange = (e) => {
    const { name, value } = e.target;
    setFormData({
      ...formData,
      [name]: value,
    });
  };

  const handleSubmit = (e) => {
    e.preventDefault();
  };

  return (
    <div>
      <section className="main-container">
        <div className="contact-container">
          <div className="contact-left-side">
            {/* <div className="left-contact-top"> */}
            <div className="contact-top-head">
              {" "}
              <div className="started">
                <img className="icon" src={icon} alt="icon" />
                <p className="started-text">Get Started</p>
              </div>
              <div className="header-texts">
                <h1 className="header-h1">Contact Us</h1>
                <p className="header-text">
                  Please fill this form to send us any questions or concerns you
                  have.
                </p>
              </div>
            </div>
            <form onSubmit={handleSubmit} className="form-part">
              <div className="form-field">
                {" "}
                <div className="label">
                  <p className="fullname">Full Name</p>
                  <input
                    className="form-input"
                    name="name"
                    value={formData.fullname}
                    onChange={handleChange}
                    type="text"
                  ></input>
                </div>
                <div className="label">
                  <p className="email">Email</p>
                  <input
                    className="form-input"
                    name="name"
                    value={formData.email}
                    onChange={handleChange}
                    type="text"
                  ></input>
                </div>
                <div className="message-label">
                  <p className="message">Message</p>
                  <input
                    className="form-message"
                    name="name"
                    value={formData.fullname}
                    onChange={handleChange}
                    type="text"
                  ></input>
                </div>
              </div>
              <button className="left-contact-button" type="submit">
                Submit
              </button>
            </form>
            {/* </div> */}
          </div>
          <div className="contact-hero-part">
            <img className="hero-image" src={contacthero} alt="hero" />
          </div>
        </div>
      </section>
    </div>
  );
}

export default ContactSection;
