import React from "react";
import effortless from "./EffortlessImage.svg";
import enhanced from "./EnhancedImage.svg";
import stunning from "./StunningImage.svg";
import action from "./ActionImage.svg";
import "./MainSection.scss";

function MainSection() {
  return (
    <div className="mainSection">
      <div className="section1">
        <h1 id="effortless1">Effortless Data Management</h1>
        <p id="effortless2">
          Basable: Simplifying data management with seamless integration and
          dynamic dashboard creation
        </p>
        <p id="effortForMobile">
          {" "}
          Simplifying data management with{" "}
          <span id="span1"> seamless dashboard creation </span>{" "}
        </p>
        <img
          src={effortless}
          alt="Effortless Data Management"
          id="effortlessImage"
        />
      </div>
      <div className="mainSectionColumn2">
        <div className="section2">
          <img src={enhanced} alt="Enhanced collaboration" id="enhancedImage" />
          <h1 id="enhanced1">Enhanced Collaboration</h1>
          <p id="enhanced2">
            {" "}
            Collaboration with centralized
            <span id="enhanced3"> dashboards for seamless insights</span>
          </p>
        </div>

        <div className="section3">
          <img
            src={stunning}
            alt="Stunning Data Visualization"
            id="stunningImage"
          />
          <h1 id="stunning1">Stunning Data Visualization </h1>
          <p id="stunning2">
            {" "}
            Link and manage your business
            <span id="stunning3"> effortlessly </span>
          </p>
        </div>
      </div>
      <div className="section4">
        <div id="action1">
          <h1 id="action2">Actionable Business Insights </h1>
          <p id="action3">
            {" "}
            Basable: Effortlessly link and manage your business data, creating
            dashboards for actionable business insights.
          </p>
          <p id="actionForMobile">
            <h1 id="mobile1">
              Actionable Business <span id="mobile2">Insights</span>{" "}
            </h1>
            <p id="mobile3">
              {" "}
              Basable: Effortless link and
              <span id="mobile4"> manage your business data, </span>
              <span id="mobile5"> creating dashboards for actionable</span>
              <span id="mobile6"> business insights.</span>
            </p>
          </p>
        </div>
        <img src={action} alt=" Actionable Business Insight" id="actionImage" />
      </div>
    </div>
  );
}

export default MainSection;
