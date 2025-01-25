#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Created on Sat Jan 25 14:37:02 2025

@author: knoedel
"""

import pandas as pd
import json

# Read the CSV file
def read_csv_file(file_path):
    # Use pandas to read the CSV file
    data = pd.read_csv(file_path, header=None, sep=";", dtype=str)  # Treat all data as strings
    return data

# Process the data
def process_csv_data(data):
    text_sections = []
    current_section = []

    for _, row in data.iterrows():
        if row.isnull().all():  # Check if the row is empty
            if current_section:  # Save the current section if it's not empty
                text_sections.append(current_section)
                current_section = []
        else:
            current_section.append(row.dropna().tolist())  # Add non-empty values of the row

    # Append the last section if it exists
    if current_section:
        text_sections.append(current_section)

    return text_sections


# Save data as a JSON file
def save_as_json(data, json_file_path):
    with open(json_file_path, "w", encoding="utf-8") as json_file:
        json.dump(data, json_file, indent=4, ensure_ascii=False)  # Write JSON with proper formatting

# Example usage
if __name__ == "__main__":
    file_path = "first_idea.csv"  # Replace with your file path
    json_file_path = "text_adventure.json"  # Output JSON file name

    # Read and process the CSV file
    data = read_csv_file(file_path)
    text_sections = process_csv_data(data)

    # Prepare data for JSON
    json_data = {"sections": []}
    for section_id, section in enumerate(text_sections):
        json_section = {"section_id": section_id, "rows": []}
        for row_id, row in enumerate(section):
            json_row = {"row_id": row_id, "values": row}
            json_section["rows"].append(json_row)
        json_data["sections"].append(json_section)

    # Save to a JSON file
    save_as_json(json_data, json_file_path)
    print(f"Data successfully saved to {json_file_path}")
