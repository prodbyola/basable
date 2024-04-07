# Basable Project Development
Our priority at the moment is to build an SLC (Simple, Lovable, Complete) Product, maybe not completely complete😀. In the next few weeks, we should have [Basable](./README.MD) that is:

* Compatible with MySQL: Users should be able to connect to a local or remote MySQL database by providing their connection credentials.

* Able to Add Basic Configurations: While creating a connection, users should be able to select a default table whose data will be ready for summary on the Dashboard. They should (optionally) be able to specify a "created_at" column (from the table) and choose a date format (for the column), which Basable will use to track the record population history.

* Users should be able to get notifications/warnings...for example, if they didn't set a "created_at" column while creating a connection or if there was an error while updating a record.

* Displays Basic Dashboard Summary for a Table: Basable should be able to provide Total Number of Records, Number of Records Added on a Specific Date, Number of Records Added Within a Date Range. Users should be able to plot two colums against each other on Bar Chart and we should be able to provide basic information using simple Pie Charts...etc.

* Able to Switch Between Tables: Users should be able to switch Dashboard for different tables available in the Database.

* Perform Basic CRUD Operation: Users should be able to add a new record, load single/multiple records, edit/update an existing record, delete a record.


## Line of Action
In the next two weeks (max), we should have a responsive page where users can initiate a connection:

* Our designers should come up with a logo and color scheme for Basable.
* Our UI/UX Designers should create a visual prototype of an Initiate New Connection (INC) page. The look and feel of the page is up to designers' discretion but we expect something beautiful and elegant, according to the Product's logo and color scheme.
* The INC page basically contains a form for collecting Database credentials. Here's a list of fields for the form:
    - A Dropdown Selection:
        - label: **Source Type**
        - options: **[RDBMS, Cloud Provider, NoSql, File]**
        - Default: **RDBMS**
        - Status: Disabled
    
    - A Dropdown Selection:
        - label: **RDBMS**
        - options: **[MySQL]**
        - Default: **MySQL**
        - Status: **Disabled**

    - A TextField
        - label: **Username**
        - Default: **root**

    - A TextField
        - label: **Password**
        - Default: **empty**

    - A TextField
        - label: **Host**
        - Default: **localhost**

    - A Number Field
        - label: **Port**
        - Default: **3306**

    - A TextField
        - label: **Database Name**
        - Default: **empty**

    - A Button for Form Submission
        - label: **Create Database**

* Our Frontend Developers will implement the UI/UX design for INC page:
    - Develop the form.
    - Handle input validations and display appropriate errors: (All fields are required except password field).
    - Connect to backend API.
    - Handle HTTP Request failure and display response errors

* Our Backend Developers will provide Restful API `POST:/connection` endpoint. This endpoint will:
    - Establish a new connection with a MySQL Database using the provided credentials.
    - Return list of tables from the table on successful connection.
    - Return appropriate connection error if there was an error.