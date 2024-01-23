import sqlite3
import pandas as pd
import matplotlib.pyplot as plt
import numpy as np

# Connect to the SQLite database
conn = sqlite3.connect("D:/santorini/rusty-santorini-engines/benchmarking/src/sql/santorini_db.db")
cursor = conn.cursor()

# Query to fetch the data including searcher names and ids
query = """
    SELECT
        s.id_searcher,
        s.nm_engine,
        sr.vl_depth,
        sr.total_entries,
        sr.avg_search_duration
    FROM
        TB_SEARCHER_INFO s
    JOIN
        vw_search_results_summary sr ON s.id_searcher = sr.id_searcher
    WHERE
        sr.id_position = 14
"""

# Execute the query and fetch the results into a DataFrame
df = pd.read_sql_query(query, conn)

# Close the database connection
conn.close()

# Pivot the DataFrame to create a grouped structure
df_pivot = df.pivot_table(index=['id_searcher', 'nm_engine'], columns='vl_depth', values='avg_search_duration', aggfunc='mean')

# Convert nanoseconds to seconds
def format_duration(duration):
    # Your existing formatting logic here...
    if duration < 1e3:
        return f'{duration:.2f} ns'
    elif duration < 1e6:
        return f'{duration / 1e3:.2f} µs'
    elif duration < 1e9:
        return f'{duration / 1e6:.2f} ms'
    elif duration < 60 * 1e9:
        return f'{duration / 1e9:.2f} s'
    elif duration < 60 * 60 * 1e9:
        return f'{duration / (60 * 1e9):.2f} min'
    else:
        return f'{duration / (60 * 60 * 1e9):.2f} hr'

# Format the DataFrame for printing with appropriate units
formatted_table = df_pivot.applymap(format_duration).to_string()

# Print the formatted table with both name and ID
print("Table:")
print(formatted_table)
