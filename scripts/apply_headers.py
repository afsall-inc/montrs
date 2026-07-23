#!/usr/bin/env python3
import os
import sys

ROOT_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
HEADER_DIR = os.path.join(ROOT_DIR, 'docs/LICENSES/headers')
HEADER_MIT_APACHE = os.path.join(HEADER_DIR, 'HEADER-MIT-APACHE')

def apply_header(file_path, header_path):
    with open(header_path, 'r') as hf:
        header_text = hf.read().strip()

    with open(file_path, 'r') as f:
        content = f.read()

    content_lines = content.splitlines(keepends=True)

    if file_path.endswith('.rs'):
        content_start_index = 0
        for i, line in enumerate(content_lines):
            stripped = line.strip()

            if stripped.startswith('//!'):
                content_start_index = i
                break
            if stripped.startswith('///'):
                content_start_index = i
                break
            if stripped.startswith('#[') or stripped.startswith('#!['):
                content_start_index = i
                break
            if (stripped.startswith('use ') or
                stripped.startswith('mod ') or
                stripped.startswith('pub ') or
                stripped.startswith('fn ') or
                stripped.startswith('struct ') or
                stripped.startswith('enum ') or
                stripped.startswith('type ') or
                stripped.startswith('impl ') or
                stripped.startswith('trait ') or
                stripped.startswith('const ') or
                stripped.startswith('static ') or
                stripped.startswith('macro_rules!')):
                content_start_index = i
                break
            if stripped.startswith('//') or not stripped:
                continue
            content_start_index = i
            break
        else:
            content_start_index = len(content_lines)

        remaining_content = content_lines[content_start_index:]

        while remaining_content and not remaining_content[0].strip():
            remaining_content.pop(0)

        new_content = header_text + '\n\n' + "".join(remaining_content)
    else:
        if content.strip().startswith(header_text):
            return False

        if content_lines and content_lines[0].strip().startswith('//'):
            end_idx = 0
            is_doc_comment = False
            for i, line in enumerate(content_lines):
                stripped = line.strip()
                if stripped.startswith('///') or stripped.startswith('//!'):
                    is_doc_comment = True
                    break
                if stripped.startswith('//') or stripped == '':
                    end_idx = i + 1
                else:
                    break

            if not is_doc_comment and end_idx > 0:
                header_candidate = "".join(content_lines[:end_idx])
                if any(indicator in header_candidate for indicator in ['Copyright', 'License', 'SPDX']):
                    content_lines = content_lines[end_idx:]

        while content_lines and content_lines[0].strip() == '':
            content_lines.pop(0)

        new_content = header_text + '\n\n' + "".join(content_lines)

    if new_content != content:
        with open(file_path, 'w') as f:
            f.write(new_content)
        return True
    return False

def main():
    for root, dirs, files in os.walk(ROOT_DIR):
        if '.git' in dirs:
            dirs.remove('.git')
        if 'target' in dirs:
            dirs.remove('target')
        if 'node_modules' in dirs:
            dirs.remove('node_modules')

        for file in files:
            if file.endswith('.rs'):
                file_path = os.path.join(root, file)
                if apply_header(file_path, HEADER_MIT_APACHE):
                    print(f"Applied header to {os.path.relpath(file_path, ROOT_DIR)}")
            elif file == 'Cargo.toml':
                file_path = os.path.join(root, file)
                with open(file_path, 'r') as f:
                    lines = f.readlines()

                new_lines = []
                changed = False
                for line in lines:
                    stripped = line.strip()
                    if stripped.startswith('license ='):
                        new_lines.append('license = "Apache-2.0 OR MIT"\n')
                        if stripped != 'license = "Apache-2.0 OR MIT"':
                            changed = True
                    else:
                        new_lines.append(line)

                if changed:
                    with open(file_path, 'w') as f:
                        f.writelines(new_lines)
                    print(f"Updated license in {os.path.relpath(file_path, ROOT_DIR)} to Apache-2.0 OR MIT")

if __name__ == "__main__":
    main()