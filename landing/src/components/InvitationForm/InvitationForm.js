import React from "react";
import "./InvitationForm.scss";
import bg from "../InvitationForm/assets/images/bg.png";
import "./InvitationForm.scss";

function InvitationForm() {
  return (
    <div className="invitation-form">
      <div className="invitation-form__background">
        <img
          src={bg}
          alt="Background"
          className="invitation-form__background-image"
        />
      </div>
      <div className="invitation-form__content">
        <h1 className="invitation-form__title">Be the First to Know!</h1>
        <p className="invitation-form__description">
          Join the waiting list to get notified when Basable launches and unlock
          exclusive benefits.
        </p>
        <div className="invitation-form__input-container">
          <input
            type="email"
            className="invitation-form__input"
            placeholder="Enter your email"
          />
          <button className="invitation-form__button">Join now</button>
        </div>
      </div>
    </div>
  );
}

export default InvitationForm;
