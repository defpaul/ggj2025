#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Created on Sat Jan 25 16:50:12 2025

@author: knoedel
"""
import pandas as pd
import json

# Read the CSV file
def read_csv_file(file_path):
    # Read CSV with ; as a delimiter
    data = pd.read_csv(file_path, header=None, sep=";", dtype=str)  # Treat all as strings
    return data

# Process the data
def process_csv_data(data):
    questions_and_answers = []  # List to store the extracted Q&A pairs
    current_block = []          # Temporary storage for each block of rows

    for _, row in data.iterrows():
        if row.isnull().all():  # Empty row indicates the end of a block
            # Process the current block if it has at least 2 rows
            if len(current_block) >= 2:
                process_block(current_block, questions_and_answers)
            current_block = []  # Reset block
        else:
            current_block.append(row.fillna(""))  # Add row to the block, replacing NaN with empty strings

    # Process any remaining data at the end of the file
    if len(current_block) >= 2:
        process_block(current_block, questions_and_answers)

    return questions_and_answers

# Helper function to process a single block
def process_block(block, questions_and_answers):
    questions_row = block[0].tolist()  # First row contains questions
    answers_row = block[1].tolist()   # Second row contains answers

    for i, question in enumerate(questions_row):
        if question.strip():  # Skip empty questions
            # Extract answers for the current question
            start_idx = i * 3
            answers = answers_row[start_idx:start_idx + 3]

            # Ensure there are at least 3 answers, filling with empty strings if needed
            answers = answers + [""] * (3 - len(answers))

            # Add question and answers to the list
            questions_and_answers.append({
                "question": question,
                "answers": [
                    {"text": answers[0], "is_correct": True},   # Correct answer
                    {"text": answers[1], "is_correct": False},  # Wrong answer 1
                    {"text": answers[2], "is_correct": False}   # Wrong answer 2
                ]
            })

# Save data as a JSON file
def save_as_json(data, json_file_path):
    with open(json_file_path, "w", encoding="utf-8") as json_file:
        json.dump(data, json_file, indent=4, ensure_ascii=False)  # Write JSON with proper formatting

# Example usage
if __name__ == "__main__":
    file_path = "first_idea.csv"  # Replace with your file path
    json_file_path = "questions_and_answers.json"  # Output JSON file name

    # Read and process the CSV file
    data = read_csv_file(file_path)
    questions_and_answers = process_csv_data(data)

    # Save to a JSON file
    save_as_json(questions_and_answers, json_file_path)
    print(f"Data successfully saved to {json_file_path}")

