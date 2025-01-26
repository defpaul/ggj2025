#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Created on Sat Jan 25 16:50:12 2025

@author: knoedel
"""

import pandas as pd
import json
import os

# Read the CSV file
def read_csv_file(file_path):
    # Read CSV with ; as a delimiter
    data = pd.read_csv(file_path, header=None, sep=";", dtype=str)  # Treat all as strings
    return data

# Generate a unique filename for each question
def generate_filename(index):
    return f"question_{index + 1}.json"

# Process the data and create separate JSON files
def process_csv_data(data, output_dir):
    current_block = []
    filenames = []  # Store filenames for linking
    index = 0  # Question index

    for row_index, row in data.iterrows():
        if row.isnull().all():  # Empty row indicates the end of a block
            if len(current_block) == 2:  # Ensure there are exactly two rows in the block
                questions_row = current_block[0].dropna().tolist()  # Get all non-NaN questions
                answers_row = current_block[1].dropna().tolist()    # Get all non-NaN answers

                # Process each question and its related answers
                for i, question in enumerate(questions_row):
                    answers = answers_row[i*3:i*3+3]  # Get 3 answers for the question
                    if len(answers) == 3:  # Ensure there are exactly 3 answers
                        # Generate a filename for the current question
                        filename = generate_filename(index)
                        filenames.append(filename)

                        # Determine the next question filenames for each answer
                        next_filenames = []
                        if row_index + 2 < len(data):  # Ensure the "next question" row exists
                            next_questions = data.iloc[row_index + 2].dropna().tolist()  # Row two below
                            next_filenames = [generate_filename(filenames.index(q)) if q in filenames else None for q in next_questions]

                        # Save question and answers to a JSON file
                        question_data = {
                            "filename": filename,
                            "person": "Harri",
                            "dialog": [
                                {"talker": "npc", "text": question}
                            ],
                            "answers": [
                                {"short": "keyent", "text": answers[0], "next": next_filenames[0] if len(next_filenames) > 0 else None},
                                {"short": "keyent", "text": answers[1], "next": next_filenames[1] if len(next_filenames) > 1 else None},
                                {"short": "keyent", "text": answers[2], "next": next_filenames[2] if len(next_filenames) > 2 else None}
                            ]
                        }

                        save_as_json(question_data, os.path.join(output_dir, filename))
                        index += 1

            current_block = []  # Reset block
        else:
            current_block.append(row)

# Save a single question-answer set as a JSON file
def save_as_json(data, file_path):
    with open(file_path, "w", encoding="utf-8") as json_file:
        json.dump(data, json_file, indent=4, ensure_ascii=False)  # Write JSON with proper formatting

# Example usage
if __name__ == "__main__":
    file_path = "first_idea.csv"  # Replace with your file path
    output_dir = "questions_json"  # Directory to save JSON files
    os.makedirs(output_dir, exist_ok=True)  # Create directory if it doesn't exist

    # Read and process the CSV file
    data = read_csv_file(file_path)
    process_csv_data(data, output_dir)

    print(f"All questions and answers have been saved as separate JSON files in '{output_dir}'")
