import { type FunctionComponent, useState } from 'react';
import { open } from '@tauri-apps/api/dialog';
import { pictureDir } from '@tauri-apps/api/path';
import { invoke } from '@tauri-apps/api';

const App: FunctionComponent = () => {
  const [filePath, setFilePath] = useState('');

  const handleOpenClick = async () => {
    const paths = await open({
      directory: false,
      multiple: true,
      defaultPath: await pictureDir(),
      filters: [
        {
          name: "Images",
          extensions: ['jpg', 'png', 'arw', 'raf', 'dng'],
        },
      ],
    });

    const metadata = await invoke("get_image_metadata", { paths: Array.isArray(paths) ? paths : [paths ?? ''] });

    setFilePath(JSON.stringify(metadata));
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
