import path from 'path';
import fs from 'fs';
import { npmDirectory } from '@/utils/dir.js';

export default async function combineExternalScripts(files: string[]) {
  const output = path.join(npmDirectory, 'src-tauri/src/inject/dynamic.js');

  const contents = files.map(file => fs.readFileSync(file, 'utf-8'));
  fs.writeFileSync(output, contents.join('\n'));
  return files;
}