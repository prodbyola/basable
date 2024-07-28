# VISUALIZING DATA WITH BASABLE

There are several ways we can analyze data with Basable, depending on our objective. For our MVP, we'll be providing two features: Graphical Visualization and Statistical Analysis. With Statistical Analysis, we'll be able to calculate metrics such as Mean, Median, Modes, Standard Deviation, Minimum and Maximum for numerical columns...etc. This is easier to implement and also, it may not be very important to our initial target audience. So this documentation will focus on the other feature. **Please note:** we'll continue to update this documentation as different requirements change.


Graphical Visualization is a bit more complex but we'll try to break it down into smaller implementations driven towards simplicity and purpose. **Our goal with visualization is to provide quick answers (insights) to certain questions (user queries)** through the use of visual components such as Bar Charts, Histogram, Pie Charts, Scatter Plot...etc. For visualization examples in this document, the y-axis will be described as dependent variables and x-axis as independent variables. We'll use an e-commerce app database as a case study but our design approach will fit just for any database. We'll also assume our database has the following tables:

* users

* vendors

* products

* orders

* categories

* rating

* reviews

Each graph type described below provides answers to certain questions and each should have its own dedicated visual component that provides insights. So it's actually possible we have multiple graph types displayed on just one dashboard. In our subsequent releases, we'll be able layer multiple graph components on top of one another. 

In this document, we'll also look at the query parameters we can provide for construct each graph. For reusability, we should be able to save and retrieve graphs' parameters along with the following configurations:

* Chart Label
* Chart Caption
* Y-Label
* X-Label

Now let's see the different graphs we can construct.

## Chrono Graphs
This graph type aims to provide insights into how data is distributed over a period of time. It's mostly useful for determining how data is being added/updated on a specific table in time. It's an attempt to answer questions such as:

* How many users were registered each day over the past one month?

* How many products were added to the store each month in the last 5 months?

* How many orders are completed on yearly basis in the last 3 years?

#### Parameters
To construct a Chrono Graph, we need to provide the following query parameters:

* table: The table to analyze
* chrono_col: The table column where we can find the timestamp.
* basis: Result grouping scope. This can be Hourly, Daily, Monthly or Yearly. (Defaults to Monthly)
* range: The time range around which we perform the analysis. (Defaults to One Month Ago to Now)

## Trend Graphs 
To track and compare how certain elements perform against others within our table, we use Trend Graphs. This graph type aims to provide answers to questions like:

* What are the top 10 products with the highest sales?
* Who are the top 15 vendors with the highest products added in the month of May?
* What are the top 10 reviews with lowest numbers of likes?

There are two types of Trend Graphs, based on how table structure and analysis configuration.
1. Intra Model: This type of Trend Graph assumes that values of dependent variables are stored in the table we analyze. For example, if we want to find top 10 product reviews with lowest numbers of likes? We assume that the number of likes is stored in a `likes` column on `reviews` table, such that if review `id` is the independent variable (x), `likes` is the dependent variable (y).

2. Cross Model: This graph type takes values of its dependent variables from another (related) table. For example, to find top 10 products with highest sales between the months of June and August, we can take the product name (from products table) as the independent variable (y) and take the order counts (associated with the product) as dependent variable (x).

Another example of Cross Model Graph is, let's say our app `vendors` table has a related `shops` table which has `views` column used for monitoring traffic to a vendor's shops, then we can analyze vendors with highest number of accumulated visitors by tracking the right column from the `shops` table.

#### Parameters
To construct a Trend Graph, we need to provide the following parameters:

* table: The table we'd like to analyze.
* analysis_type: The type of analysis we'd like to perform. This could be Intra Model or Cross Model analysis.
* xcol: The column to use for independent variables of the graph.
* ycol: The column to use for dependent variables of the graph.
* order: The order of the analysis. This could be ASC for ascending order or DESC for descending order.
* limit: Total number of returned data.
* cross: options to be set if our graph type is Cross Model.

## Categoric Graphs
This type of graphs provide insights into how data is distributed over groups. It can answer questions like:

* How many products do we have in "Men's Wear" category?
* How many users are in between age 10-15 or age 20-22?

There are 3 analysis types we can perform to construct a category graph: 

Simple: With simple analysis, we create groups from a column on the table we're analyzing. The groups are the independent variables while their respective counts become the dependent variables. For example, let's we have a `products` table which has a `color` column. We can group our products according to their respective color and construct a frequency graph for the groups.

Other analysis types are `ManyToMany` and `Manual`. We'll update this documentation with their descriptions after they're implemented. For a start, we'll be implementing `Simple` analysis type.

Parameters
To construct a (simple) categoric graph, we can supply the following parameters:

* table: The table we wish to analyze.
* analysis_type: The type of analysis to perform. Available options are `simple`, `many` and `manual`.
* target_column: The column to target for grouping.
* limit: Optional limit to place on the results.

## Geo Graphs
Geo Graphs are somehow like Categoric Graphs but with group focused on geographic locations. It provides insights into how our data is distributed across geographic regions. It can answer questions like:

* How many of our users are in Africa?
* How many products were sold in Canada?
* Which state in Nigeria has the highest number of vendors?

#### Graph Scope
We can determine the scope of our Geographic Graph by specifying one of the following options:
* global: analyzes the whole world
* continent: analyzes a specific continent
* nation: analyzes a nation or country
* regions: analyzes manually specified regions.

Parameters
To construct a (simple) Geo Graph, we can supply the following parameters:
* table: The table to analyze.
* scope: scope of the analysis as descibed above.
* target_column: column where locations are stored.