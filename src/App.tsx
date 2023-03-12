import { type FunctionComponent, useState } from 'react';
import { open } from '@tauri-apps/api/dialog';
import { pictureDir } from '@tauri-apps/api/path';
import { invoke } from '@tauri-apps/api';

const App: FunctionComponent = () => {
  const [filePath, setFilePath] = useState('');

  const handleOpenClick = async () => {
    let path = await open({
      directory: true,
      multiple: false,
      defaultPath: await pictureDir(),
    });

    if (Array.isArray(path)) {
      path = path[0];
    }

    try {
      const metadata = await invoke('add_to_library', {
        rootPath: path,
        recursive: false,
      });

      setFilePath(JSON.stringify(metadata));
    } catch (e) {
      console.error(e);
    }
  };

  return (
    <div className="container mx-auto">
      <h1 className="text-5xl font-bold">Welcome to Tauri!</h1>

      <div className="row">
        <button
          className="btn btn-primary btn-lg"
          type="button"
          onClick={handleOpenClick}
        >
          Open
        </button>
      </div>
      <p>{filePath}</p>
    </div>
  );
};

export default App;
