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
    data = pd.read_csv(file_path, header=None, sep=";", dtype=str)  # Treat all as strings
    return data

# Generate a valid filename from a question string
def generate_filename(question):
    sanitized = "".join(c if c.isalnum() else "_" for c in question[:30])  # Max 30 chars, replace non-alphanumeric
    return f"{sanitized}.json"

# Process the data
def process_csv_data(data):
    questions_and_answers = []
    current_block = []
    all_questions = {}

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
                                next_filename = generate_filename(next_question)
                                next_questions.append(next_filename)
                            else:
                                next_questions.append(None)  # No next question

                        # Save the question and its answers
                        questions_and_answers.append({
                            "person": "Harri",  # Fixed NPC name
                            "dialog": [
                                {"talker": "npc", "text": question}
                            ],
                            "answers": [
                                {"short": "keyent", "text": answers[0], "next": next_questions[0]},
                                {"short": "keyent", "text": answers[1], "next": next_questions[1]},
                                {"short": "keyent", "text": answers[2], "next": next_questions[2]},
                            ]
                        })

                        # Map question to filename for JSON creation later
                        all_questions[question] = generate_filename(question)

            current_block = []  # Reset block
        else:
            current_block.append(row)

    # Process the last block if still in memory
    if len(current_block) == 2:
        questions_row = current_block[0].dropna().tolist()
        answers_row = current_block[1].dropna().tolist()

        for i, question in enumerate(questions_row):
            answers = answers_row[i * 3:i * 3 + 3]
            if len(answers) == 3:
                next_questions = []
                for j, answer in enumerate(answers):
                    next_row_index = len(data) - 1 + (j * 2)
                    if next_row_index < len(data) and not data.iloc[next_row_index].isnull().all():
                        next_question = data.iloc[next_row_index].dropna().iloc[0]
                        next_filename = generate_filename(next_question)
                        next_questions.append(next_filename)
                    else:
                        next_questions.append(None)

                questions_and_answers.append({
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
                all_questions[question] = generate_filename(question)

    return questions_and_answers, all_questions

# Save JSON files for each question
def save_questions_as_json(questions_and_answers, base_dir):
    if not os.path.exists(base_dir):
        os.makedirs(base_dir)

    for question_data in questions_and_answers:
        question_text = question_data["dialog"][0]["text"]
        filename = generate_filename(question_text)
        filepath = os.path.join(base_dir, filename)
        with open(filepath, "w", encoding="utf-8") as json_file:
            json.dump(question_data, json_file, indent=4, ensure_ascii=False)

# Main program
if __name__ == "__main__":
    file_path = "first_idea.csv"  # Replace with your CSV file path
    output_dir = "json_files"  # Directory for the JSON files

    # Read and process the CSV file
    data = read_csv_file(file_path)
    questions_and_answers, all_questions = process_csv_data(data)

    # Save each question as a separate JSON file
    save_questions_as_json(questions_and_answers, output_dir)

    print(f"All JSON files have been successfully saved to '{output_dir}'.")