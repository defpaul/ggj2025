#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Created on Sat Jan 25 15:58:07 2025

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
    current_block = []

    for _, row in data.iterrows():
        if row.isnull().all():  # Empty row indicates the end of a block
            if len(current_block) == 2:  # Ensure there are exactly two rows in the block
                questions_row = current_block[0].dropna().tolist()  # Get all non-NaN questions
                answers_row = current_block[1].dropna().tolist()    # Get all non-NaN answers
                
                # Process questions and their corresponding answers
                for i, question in enumerate(questions_row):
                    answers = answers_row[i*3:i*3+3]  # Get 3 answers for the question
                    if len(answers) == 3:  # Ensure there are exactly 3 answers
                        questions_and_answers.append({
                            "question": question,
                            "answers": [
                                {"short":"jumpmark", "text": answers[0], "next": "filename"},   
                                {"short":"jumpmark", "text": answers[1], "next": "filename"},  
                                {"short":"jumpmark", "text": answers[2], "next": "filename"}  
                            ]
                        })
            current_block = []  # Reset block
        else:
            current_block.append(row)
            
    # Process any remaining data in current_block after the loop ends
    if len(current_block) == 2:  # If the last block is complete
        questions_row = current_block[0].dropna().tolist()
        answers_row = current_block[1].dropna().tolist()

        # Process questions and their corresponding answers
        for i, question in enumerate(questions_row):
            answers = answers_row[i*3:i*3+3]
            if len(answers) == 3:
                questions_and_answers.append({
                    "question": question,
                    "answers": [
                        {"short":"jumpmark", "text": answers[0], "next": "filename"},   
                        {"short":"jumpmark", "text": answers[1], "next": "filename"},  
                        {"short":"jumpmark", "text": answers[2], "next": "filename"}  
                    ]
                })


    return questions_and_answers

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
