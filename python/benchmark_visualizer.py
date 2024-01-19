import sqlite3
import pandas as pd
import matplotlib.pyplot as plt
import numpy as np

# Connect to the SQLite database
conn = sqlite3.connect("D:/santorini/rusty-santorini-engines/benchmarking/src/sql/santorini_db.db")
cursor = conn.cursor()

# Query to fetch the data including searcher names
query = """
    SELECT
        s.id_searcher,
        s.nm_engine,
        sr.id_position,
        sr.vl_depth,
        sr.total_entries,
        sr.avg_search_duration
    FROM
        TB_SEARCHER_INFO s
    JOIN
        vw_search_results_summary sr ON s.id_searcher = sr.id_searcher
    WHERE
        sr.id_position = 1
"""

# Execute the query and fetch the results into a DataFrame
df = pd.read_sql_query(query, conn)

# Close the database connection
conn.close()

# Pivot the DataFrame to create a grouped structure
df_pivot = df.pivot_table(index='nm_engine', columns='vl_depth', values='avg_search_duration', aggfunc='mean')

# Convert nanoseconds to seconds
df_pivot = df_pivot / 1e9

# Create a grouped bar plot with logarithmic scale
fig, ax = plt.subplots()

bar_width = 0.2  # Adjust the width of each bar
engines = df_pivot.index
num_depths = len(df_pivot.columns)

for i, (engine, row) in enumerate(df_pivot.iterrows()):
    positions = np.arange(num_depths) + i * bar_width
    ax.bar(positions, row, bar_width, label=engine)

# Set plot labels and title
ax.set_xlabel('Depth')
ax.set_ylabel('Average Search Duration (seconds)')
ax.set_title('Performance Comparison of Searchers in Position 1')
ax.set_yscale('log')  # Set logarithmic scale on the y-axis

# Set x-axis ticks and labels
ax.set_xticks(np.arange(num_depths) + (num_depths - 1) * bar_width / 2)
ax.set_xticklabels(df_pivot.columns)

# Add legend
ax.legend(title='Searcher')

# Show the plot
plt.show()
