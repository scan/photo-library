import { useState, type FunctionComponent } from 'react';
import cn from 'classnames';
import { open } from '@tauri-apps/api/dialog';
import { pictureDir } from '@tauri-apps/api/path';
import { useMutation } from '@tanstack/react-query';

const noop = () => {};

const findRootPath = async (): Promise<string | undefined> => {
  let path = await open({
    directory: true,
    multiple: false,
    defaultPath: await pictureDir(),
  });

  if (Array.isArray(path)) {
    path = path[0];
  }

  return path ?? undefined;
};

interface Props {
  open?: boolean;
  onClose?: () => void;
}

const AddImagesModal: FunctionComponent<Props> = ({
  open = false,
  onClose = noop,
}) => {
  const fetchRootPathMutation = useMutation(findRootPath);

  const handleChoosePathClick = () => {
    fetchRootPathMutation.mutate();
  };

  return (
    <div className={cn('modal', { 'modal-open': open })}>
      <div className="modal-box">
        <h3 className="font-bold text-lg">Add Photos to Library</h3>
        <div className="py-4">
          <div className="form-control">
            <div className="input-group w-full">
              <input
                type="text"
                placeholder="Photo Directory"
                className="input input-bordered"
                readOnly
                value={fetchRootPathMutation.data}
              />
              <button
                className={cn('btn btn-secondary', {
                  'btn-disabled': fetchRootPathMutation.isLoading,
                })}
                disabled={fetchRootPathMutation.isLoading}
                type="button"
                onClick={handleChoosePathClick}
              >
                Choose
              </button>
            </div>
          </div>
        </div>
        <div className="modal-action">
          <button className="btn btn-secondary" type="button" onClick={onClose}>
            Cancel
          </button>
          <button
            className={cn('btn btn-primary', {
              'btn-disabled': !fetchRootPathMutation.data,
            })}
            type="submit"
          >
            Start
          </button>
        </div>
      </div>
    </div>
  );
};

export default AddImagesModal;
