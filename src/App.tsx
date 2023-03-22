import { type FunctionComponent, useState } from 'react';
import { open } from '@tauri-apps/api/dialog';
import { pictureDir } from '@tauri-apps/api/path';
import { invoke } from '@tauri-apps/api';
import { useMutation } from '@tanstack/react-query';

import { initiateLibraryAdd } from './api';

const App: FunctionComponent = () => {
  const startAddingMutation = useMutation(initiateLibraryAdd);

  const handleOpenClick = async () => {
    let path = await open({
      directory: true,
      multiple: false,
      defaultPath: await pictureDir(),
    });

    if (Array.isArray(path)) {
      path = path[0];
    }

    if (!path) {
      return;
    }

    startAddingMutation.mutate({ rootPath: path, recursive: false });
  };

  return (
    <div className="container mx-auto">
      <h1 className="text-5xl font-bold">Welcome to Tauri!</h1>

      <div className="row">
        <button
          className="btn btn-primary btn-lg"
          type="button"
          disabled={startAddingMutation.isLoading}
          onClick={handleOpenClick}
        >
          Open
        </button>
      </div>
    </div>
  );
};

export default App;
