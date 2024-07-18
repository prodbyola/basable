import React, { useEffect, useState, useRef } from 'react';
import './Benefit.scss';
import symbolDatabaseLeft from './assets/symbol-database-left.svg';
import symbolDatabaseRight from './assets/symbol-database-right.svg';
import benefitIcon from './assets/benefit-icon.svg';
import freedom from './assets/freedom.svg';
import solution from './assets/solution.svg';
import insights from './assets/insights.svg';

const Benefit = () => {
  const [activeLed, setActiveLed] = useState(1);
  const tabletContentRef = useRef(null);

  useEffect(() => {
    const handleScroll = () => {
      if (tabletContentRef.current) {
        const scrollLeft = tabletContentRef.current.scrollLeft;
        const scrollWidth = tabletContentRef.current.scrollWidth;
        const clientWidth = tabletContentRef.current.clientWidth;

        // Calculate the active LED based on scroll position
        const totalScrollableWidth = scrollWidth - clientWidth;
        const scrollPercentage = scrollLeft / totalScrollableWidth;

        if (scrollPercentage < 0.33) {
          setActiveLed(1);
        } else if (scrollPercentage < 0.66) {
          setActiveLed(2);
        } else {
          setActiveLed(3);
        }
      }
    };

    const tabletContentEl = tabletContentRef.current;
    tabletContentEl.addEventListener('scroll', handleScroll);

    return () => {
      tabletContentEl.removeEventListener('scroll', handleScroll);
    };
  }, []);

  const handleLedClick = (index) => {
    if (tabletContentRef.current) {
      const scrollWidth = tabletContentRef.current.scrollWidth;
      const clientWidth = tabletContentRef.current.clientWidth;
      const totalScrollableWidth = scrollWidth - clientWidth;
      const newScrollLeft = (totalScrollableWidth / 3) * (index - 1);
      tabletContentRef.current.scrollTo({
        left: newScrollLeft,
        behavior: 'smooth'
      });
      setActiveLed(index);
    }
  };

  return (
    <section id="benefit" className="benefit">
      <div className="symbol-container">
        <img
          src={symbolDatabaseLeft}
          alt="Symbol Database Left"
          className="symbol-database-left"
        />
        <img
          src={symbolDatabaseRight}
          alt="Symbol Database Right"
          className="symbol-database-right"
        />
      </div>
      <div className="benefit-title">
        <img src={benefitIcon} alt="Benefit Icon" className="benefit-icon" />
        <span>Why Our Product Is Different</span>
      </div>
      <div className="benefit-content">
        <div className="benefit-card">
          <img src={freedom} alt="freedom" />
          <span className="card-title">Open-Source Freedom</span>
          <span className="card-text">
            Empower your team with complete control
            <br /> and customization. No vendor lock-in.
          </span>
        </div>
        <div className="line-group">
          <div className="line-1"></div>
          <div className="line-2"></div>
          <div className="line-3"></div>
          <div className="line-4"></div>
        </div>
        <div className="benefit-card  left-bottom-card">
          <img src={solution} alt="solution" />
          <span className="card-title">Cost Effective Solution</span>
          <span className="card-text">
            Leverage the power of open-source and
            <br /> avoid expensive commercial licenses.
          </span>
        </div>
        <div className="benefit-card right-bottom-card">
          <img src={insights} alt="insights" />
          <span className="card-title">Collaboration Insights</span>
          <span className="card-text">
            Share dashboards and facilitate data-
            <br />
            driven decision making.
          </span>
        </div>
      </div>

      <div className="benefit-content-tablet" ref={tabletContentRef}>
        <div className="benefit-card card-1">
          <img src={freedom} alt="freedom" />
          <span className="card-title">Open-Source Freedom</span>
          <span className="card-text">
            Empower your team with complete
            <br />
            control and customization.
            <br /> No vendor lock-in.
          </span>
        </div>

        <div className="benefit-card card-2">
          <img src={solution} alt="solution" />
          <span className="card-title">Cost Effective Solution</span>
          <span className="card-text">
            Leverage the power of open-
            <br />
            source and avoid expensive
            <br /> commercial licenses.
          </span>
        </div>
        <div className="benefit-card card-3">
          <img src={insights} alt="insights" />
          <span className="card-title">Collaboration Insights</span>
          <span className="card-text">
            Share dashboards and facilitate
            <br /> data-driven decision
            <br /> making.
          </span>
        </div>
      </div>
      <div className="led-group">
        <div
          className={`${activeLed === 1 ? 'active' : ''}`}
          onClick={() => handleLedClick(1)}
        ></div>
        <div
          className={`${activeLed === 2 ? 'active' : ''}`}
          onClick={() => handleLedClick(2)}
        ></div>
        <div
          className={`${activeLed === 3 ? 'active' : ''}`}
          onClick={() => handleLedClick(3)}
        ></div>
      </div>
    </section>
  );
};

export default Benefit;
