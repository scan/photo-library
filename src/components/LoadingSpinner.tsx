import { type FunctionComponent } from 'react';
import cn from 'classnames';

interface Props {
  className?: string;
}

const LoadingSpinner: FunctionComponent<Props> = ({ className }) => {
  return (
    <div className={cn('flex items-center justify-center w-8 h-8', className)}>
      <div
        className="inline-block h-full w-full animate-spin rounded-full border-4 border-solid border-current border-r-transparent align-[-0.125em] motion-reduce:animate-[spin_1.5s_linear_infinite]"
        role="status"
      >
        <span className="!absolute !-m-px !h-px !w-px !overflow-hidden !whitespace-nowrap !border-0 !p-0 ![clip:rect(0,0,0,0)]">
          Loading&hellip;
        </span>
      </div>
    </div>
  );
};

export default LoadingSpinner;
