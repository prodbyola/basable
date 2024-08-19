declare module '*.png' {
  // const content: React.FunctionComponent<React.SVGAttributes<SVGElement>>;

  const content: string;
  export default content;
}
declare module '*.svg' {
  // const content: React.FunctionComponent<React.SVGAttributes<SVGElement>>;
  import * as React from 'react';
  export const ReactComponent: React.FC<React.SVGProps<SVGSVGElement>>;
  const content: string;
  export default content;
}
