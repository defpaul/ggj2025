#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Created on Sat Jan 25 16:50:12 2025

@author: knoedel
"""

import os
import pandas as pd
import json

# Generate a unique filename for each question
def generate_filename(index):
    return f"question_{index:03d}.json"

# Save data as a JSON file
def save_as_json(data, json_file_path):
    with open(json_file_path, "w", encoding="utf-8") as json_file:
        json.dump(data, json_file, indent=4, ensure_ascii=False)

# Process a single block of questions and answers
def process_block(current_block, index, filenames):
    questions_row = current_block[0].dropna().tolist()  # Get all non-NaN questions
    answers_row = current_block[1].dropna().tolist()    # Get all non-NaN answers
    block_data = []

    for i, question in enumerate(questions_row):
        answers = answers_row[i * 3:i * 3 + 3]  # Get 3 answers for the question
        if len(answers) == 3:  # Ensure there are exactly 3 answers
            filename = generate_filename(index)
            block_data.append({
                "filename": filename,
                "person": "Harri",
                "place": "planet earth",
                "dialog": [
                    {"talker": "npc", "text": question}
                ],
                "answers": [
                    {"short": "keyent", "text": answers[0], "next": None},  # Placeholder
                    {"short": "keyent", "text": answers[1], "next": None},  # Placeholder
                    {"short": "keyent", "text": answers[2], "next": None}   # Placeholder
                ]
            })
            filenames[question] = filename  # Map question to its filename
            index += 1
    return block_data, index

# Post-process answers to link "next" questions
def link_next_questions(data, filenames):
    for row_index, row in data.iterrows():
        if row.isnull().all() and row_index + 2 < len(data):  # Check for valid "next questions" row
            next_questions_row = data.iloc[row_index + 2].dropna().tolist()  # Get the next questions
            current_questions_row = data.iloc[row_index - 2].dropna().tolist()  # Questions in the current block

            # Assign "next" fields for the current block
            for question in current_questions_row:
                if question in filenames:
                    json_data_path = filenames[question]
                    with open(os.path.join(output_dir, json_data_path)) as json_file:
                        json_data = json.load(json_file)

                    for i, answer in enumerate(json_data["answers"]):
                        if i < len(next_questions_row):  # Ensure there's a next question
                            next_question = next_questions_row[i]
                            if next_question in filenames:
                                answer["next"] = filenames[next_question]

                    
                    print(f"Linking question: {question}")
                    print(f"Next question: {next_question} -> File: {filenames.get(next_question, 'NOT FOUND')}")

                    # Save the updated JSON file
                    save_as_json(json_data, os.path.join(output_dir, json_data_path))

# Process the entire CSV file and create JSON files
def process_csv_data(data, output_dir):
    current_block = []
    filenames = {}  # Map of question to its filename
    index = 0  # Question index

    for row_index, row in data.iterrows():
        if row.isnull().all():  # Empty row indicates the end of a block
            if len(current_block) == 2:  # Ensure there are exactly two rows in the block
                block_data, index = process_block(current_block, index, filenames)
                for entry in block_data:
                    save_as_json(entry, os.path.join(output_dir, entry["filename"]))
            current_block = []  # Reset block
        else:
            current_block.append(row)

    # Process the last block if it exists
    if len(current_block) == 2:
        block_data, index = process_block(current_block, index, filenames)
        for entry in block_data:
            save_as_json(entry, os.path.join(output_dir, entry["filename"]))

    # Link the "next" questions
    link_next_questions(data, filenames)

# Main function
if __name__ == "__main__":
    file_path = "first_idea.csv"  # Replace with your file path
    output_dir = "output_jsons"  # Directory to save JSON files

    # Create output directory if it doesn't exist
    os.makedirs(output_dir, exist_ok=True)

    # Read the CSV file
    data = pd.read_csv(file_path, header=None, sep=";", dtype=str)

    # Process and generate JSON files
    process_csv_data(data, output_dir)
    print(f"JSON files created in {output_dir}")
