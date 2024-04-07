# Basable

Basable is an open-source software designed to provide comprehensive data management, data visualization, and business insights. The core project is written in [Rust](https://www.rust-lang.org/), offering robust performance, data integrity and reliability.

## Features

- **Data Management**: Basable allows users to efficiently manage large datasets from multiple sources - Relational Databases, NoSQL, Cloud Databases or Files (CSV, Feather, Json...etc). Perform basic CRUD operations. Import and export data. Perform complex queries. Basable provides a seamless experience for transporting, transforming and handling data.

- **Data Visualization**: Visualize your data in meaningful ways with Basable's powerful visualization tools. Create interactive charts, graphs, and dashboards to gain valuable insights from your data.

- **Business Insight**: Basable offers tools for analyzing business data and extracting actionable insights. Whether you're tracking key performance indicators or identifying trends, Basable helps you make informed decisions.

- **Manage Database Connections**: Basable helps you manage multiple database connections from a single account. There's no need to remeber all your login credentials. With a single Basable account, you can manage your connections and data management projects.

- **Enhanced Collaboration**: Collaborate easily with your developers, data analysists, marketing team and all the different stakeholders. Basable facilitates perfect teamwork on your data management.

- **Monitoring and Reports**: Effortlessly collaborate with developers, data analysts, marketing teams, and other stakeholders using Basable. Our platform fosters seamless teamwork, streamlining data management processes for enhanced productivity.

- **Traffic Monitor**: Smoothly integrate Basable into your websites and webpages to track web traffic, activities, and usage patterns. Gain valuable insights into your website visitors' behavior, empowering you to make informed decisions to enhance user experience and optimize performance.

## Setting Up Development Environment

Basable's core is written in [Rust Programming Language](https://www.rust-lang.org/) and [Axum](https://github.com/tokio-rs/axum) for `HttpService`, while the web frontend is powered by [Vue Javascript Framework](https://vuejs.org/). You need to setup your local environment for development in order to be able to run Basable locally and make your contributions.

1. Install the latest versions of Rust and NodeJs. Checkout [Install Rust](https://www.rust-lang.org/tools/install) and [Download Node.js](https://nodejs.org/en/download) for their respective guides.

2. Fork the repository and clone your forked version:
```bash 
git clone https://github.com/your-username/basable.git
```

3. Ensure you have installed Rust. Then navigate to `core` folder and run the core program:
```bash
cd basable/core && cargo run
```
&emsp; This should download all dependecies and run the program in development mode.
**NOTE:** If you would like to enable hot-reload, please install [cargo-watch](https://crates.io/crates/cargo-watch) and run `cargo watch -x run` instead. This reload the program when you make a change and any save core file.

4. Open another terminal. Ensure NodeJs is installed. Then navigate to the `web` and install javascript dependencies:

```bash
cd basable/web && npm install
```

**NOTE:** You can use any package manager of your choice e.g, [yarn](https://classic.yarnpkg.com/lang/en/docs/install/), [bun](https://bun.sh/docs/installation)...etc

Then start the development environment:
```bash
npm run dev
```

5. Once you make a change you'd want us to merge into Basable, please feel free to submit a Pull Request.

### Database Setup
You may also need to to setup a MySQL database for testing. But your current setup will do for now.

## Contact
For questions or support, please contact [prodbyola@gmail.com](mailto:prodbyola@gmail.com).