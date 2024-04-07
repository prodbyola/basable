# Basable

Basable is an open-source software designed to provide comprehensive data management, data visualization, and business insights. The core project is written in [Rust](https://www.rust-lang.org/), offering robust performance, data integrity and reliability.

## Features

- **Data Management**: Basable allows users to efficiently manage large datasets from multiple sources - [Relational Databases](https://www.oracle.com/ng/database/what-is-a-relational-database/#:~:text=In%20a%20relational%20database%2C%20each,the%20relationships%20among%20data%20points.), NoSQL, Cloud Databases or Files (CSV, Feather, Json...etc). Perform basic CRUD operations. Import and export data. Perform complex queries. Basable provides a seamless experience for transporting, transforming and handling data.

- **Data Visualization**: Visualize your data in meaningful ways with Basable's powerful visualization tools. Create interactive charts, graphs, and dashboards to gain valuable insights from your data.

- **Business Insight**: Basable offers tools for analyzing business data and extracting actionable insights. Whether you're tracking key performance indicators or identifying trends, Basable helps you make informed decisions.

- **Manage Database Connections**: Basable simplifies the management of multiple database connections through a single account. Say goodbye to the hassle of juggling numerous login credentials. With just one Basable account, you gain centralized control over your connections and data management projects.

- **Data Security**: We offer a secure data management solution. Basable ensures the privacy of your data by abstaining from collection or inspection, and by implementing measures to restrict access to authenticated users only.

- **Enhanced Collaboration**: Effortlessly collaborate with developers, data analysts, marketing teams, and other stakeholders using Basable. Our platform fosters seamless teamwork, streamlining data management processes for enhanced productivity.

- **Monitoring and Reports**: Configure event triggers to emit signals based on specific operations within your database source. These events can be monitored and reported using your preferred SMTP service.

- **Traffic Monitor**: Smoothly integrate Basable into your websites and webpages to track web traffic, activities, and usage patterns. Gain valuable insights into your website visitors' behavior, empowering you to make informed decisions to enhance user experience and optimize performance.

- **Deploy Anyhow**: The flexibility of Basable allows you to choose between installing it on your local device or deploying it to a remote server. This empowers you to access your data wherever you go, giving you the freedom to select the option that best suits your needs.

## Setting Up Development Environment

Basable's core is written in [Rust Programming Language](https://www.rust-lang.org/) and [Axum](https://github.com/tokio-rs/axum) for `HttpService`, while the web frontend is powered by [Vue Javascript Framework](https://vuejs.org/). You need to setup your local environment for development in order to be able to run Basable locally and make your contributions.

#### 1. Install Requirements
Install the latest versions of Rust and NodeJs. Checkout [Install Rust](https://www.rust-lang.org/tools/install) and [Download Node.js](https://nodejs.org/en/download) for their respective guides.

#### 2. Fork Repository
Fork the repository and clone your forked version:
```bash 
git clone https://github.com/your-username/basable.git
```

#### 3. Install Core Component
Ensure you have installed Rust. Then navigate to `core` folder and run the core program:
```bash
cd basable/core && cargo run
```
This should download all dependecies and run the program in development mode.  
**NOTE:** If you would like to enable hot-reload, please install [cargo-watch](https://crates.io/crates/cargo-watch) and run `cargo watch -x run` instead. This will automatically reload the program whenever you make a change and save any core file.

#### 4. Install Web Component
4. Open another terminal. Ensure NodeJs is installed. Then navigate to the `web` and install javascript dependencies:

```bash
cd basable/web && npm install
```

**NOTE:** You can use any package manager of your choice e.g, [yarn](https://classic.yarnpkg.com/lang/en/docs/install/), [bun](https://bun.sh/docs/installation)...etc

Then start the development environment:
```bash
npm run dev
```

#### 5. Make Changes and Submit
Once you make a change you'd want us to merge into Basable, please feel free to submit a Pull Request.

#### 6. Database Setup (Optional)
You may also need to to setup a MySQL database for testing. But your current setup will do for now.

## Contact
For questions or support, please contact [prodbyola@gmail.com](mailto:prodbyola@gmail.com).