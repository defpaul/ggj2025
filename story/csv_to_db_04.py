#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Created on Sat Jan 25 21:12:05 2025

@author: knoedel
"""

import pandas as pd
import json
import os

# Read the CSV file
def read_csv_file(file_path):
    data = pd.read_csv(file_path, header=None, sep=";", dtype=str)  # Treat all as strings
    return data

# Generate a unique filename based on a numbering system
def generate_filename(counter):
    return f"{counter:05}.json"  # 5-digit zero-padded number

# Process the data
def process_csv_data(data):
    questions_and_answers = []
    current_block = []
    all_questions = {}
    counter = 1  # Start numbering files from 00001

    # First pass: Store all questions and assign them filenames
    def get_or_create_filename(question):
        """Ensure every question has a unique filename."""
        nonlocal counter
        if question not in all_questions:
            all_questions[question] = generate_filename(counter)
            counter += 1
        return all_questions[question]

    for row_index, row in data.iterrows():
        if row.isnull().all():  # Empty row indicates the end of a block
            if len(current_block) == 2:  # Ensure there are exactly two rows in the block
                questions_row = current_block[0].dropna().tolist()
                answers_row = current_block[1].dropna().tolist()

                for i, question in enumerate(questions_row):
                    get_or_create_filename(question)  # Ensure filename is assigned for the question

            current_block = []  # Reset block
        else:
            current_block.append(row)

    # Second pass: Link answers to next questions with filenames
    for row_index, row in data.iterrows():
        if row.isnull().all():  # Empty row indicates the end of a block
            if len(current_block) == 2:  # Ensure there are exactly two rows in the block
                questions_row = current_block[0].dropna().tolist()
                answers_row = current_block[1].dropna().tolist()

                for i, question in enumerate(questions_row):
                    answers = answers_row[i * 3:i * 3 + 3]  # Get 3 answers for the question
                    if len(answers) == 3:
                        # Determine next questions for each answer
                        next_questions = []
                        for j, answer in enumerate(answers):
                            next_row_index = row_index + 1 + (j * 2)  # Next question is two rows below each answer
                            if next_row_index < len(data) and not data.iloc[next_row_index].isnull().all():
                                next_question = data.iloc[next_row_index].dropna().iloc[0]
                                next_questions.append(all_questions.get(next_question, None))  # Get filename for next question
                            else:
                                next_questions.append(None)  # No next question

                        # Save the question and its answers
                        filename = all_questions.get(question)
                        questions_and_answers.append({
                            "filename": filename,
                            "person": "Harri",
                            "dialog": [
                                {"talker": "npc", "text": question}
                            ],
                            "answers": [
                                {"short": "keyent", "text": answers[0], "next": next_questions[0]},
                                {"short": "keyent", "text": answers[1], "next": next_questions[1]},
                                {"short": "keyent", "text": answers[2], "next": next_questions[2]},
                            ]
                        })

            current_block = []  # Reset block
        else:
            current_block.append(row)

    return questions_and_answers

# Save JSON files for each question
def save_questions_as_json(questions_and_answers, base_dir):
    if not os.path.exists(base_dir):
        os.makedirs(base_dir)

    for question_data in questions_and_answers:
        filename = question_data.pop("filename")  # Get the filename and remove it from the data
        filepath = os.path.join(base_dir, filename)
        with open(filepath, "w", encoding="utf-8") as json_file:
            json.dump(question_data, json_file, indent=4, ensure_ascii=False)

# Main program
if __name__ == "__main__":
    file_path = "first_idea.csv"  # Replace with your CSV file path
    output_dir = "json_files"  # Directory for the JSON files

    # Read and process the CSV file
    data = read_csv_file(file_path)
    questions_and_answers = process_csv_data(data)

    # Save each question as a separate JSON file
    save_questions_as_json(questions_and_answers, output_dir)

    print(f"All JSON files have been successfully saved to '{output_dir}'.")