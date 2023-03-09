import { type FunctionComponent, useState } from 'react';
import { open } from '@tauri-apps/api/dialog';
import { appDataDir } from '@tauri-apps/api/path';

const App: FunctionComponent = () => {
  const [filePath, setFilePath] = useState('');

  const handleOpenClick = async () => {
    const path = await open({
      directory: true,
      multiple: true,
      defaultPath: await appDataDir(),
    });

    setFilePath(Array.isArray(path) ? path.join(';') : path ?? '');
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
