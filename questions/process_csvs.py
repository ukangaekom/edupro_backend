import os
import csv

base_dir = "/home/ekomabasi/Development/Freelance/edupro_backend/questions"
folders = ["biology", "english"]

for folder in folders:
    folder_path = os.path.join(base_dir, folder)
    if not os.path.exists(folder_path):
        print(f"Folder {folder_path} does not exist.")
        continue
    for filename in os.listdir(folder_path):
        if filename.endswith(".csv"):
            filepath = os.path.join(folder_path, filename)
            
            with open(filepath, 'r', newline='', encoding='utf-8') as f:
                reader = csv.reader(f)
                rows = list(reader)
            
            if not rows:
                print(f"Empty file: {filepath}")
                continue
                
            header = rows[0]
            
            # Check if columns already exist
            has_subject = 'subject' in header
            has_exam_type = 'exam_type' in header
            
            if not has_subject:
                header.append('subject')
            if not has_exam_type:
                header.append('exam_type')
                
            subject_idx = header.index('subject')
            exam_type_idx = header.index('exam_type')
            
            for row in rows[1:]:
                # pad row if necessary
                while len(row) < max(subject_idx, exam_type_idx) + 1:
                    row.append("")
                row[subject_idx] = folder
                row[exam_type_idx] = 'Jamb'
                
            with open(filepath, 'w', newline='', encoding='utf-8') as f:
                writer = csv.writer(f, quoting=csv.QUOTE_MINIMAL)
                writer.writerows(rows)
            print(f"Processed {filepath}")
