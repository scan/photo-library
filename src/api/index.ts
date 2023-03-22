import { invoke } from '@tauri-apps/api';

interface InitiateLibraryAddOptions {
  rootPath: string;
  recursive: boolean;
}

export const initiateLibraryAdd = async ({ rootPath, recursive }: InitiateLibraryAddOptions): Promise<void> => {
  await invoke('add_to_library', {
    rootPath,
    recursive,
  });
}
