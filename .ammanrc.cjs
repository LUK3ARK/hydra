// @ts-check
'use strict';
const path = require('path');

const remoteDeployDir = path.join(__dirname, "../metaplex-program-library", 'target', 'deploy');

const programIds = {
  metadata: 'metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s',
  fanout: 'hyDQ4Nz1eYyegS6JfenyKwKzYxRsCWCriYSAjtzP4Vg'
};

function localDeployPath(dir, programName) {
  return path.join(dir, `${programName}.so`);
}
const programs = {
  metadata: { programId: programIds.metadata, deployPath: localDeployPath(remoteDeployDir, 'mpl_token_metadata') },
  fanout: { programId: programIds.fanout, deployPath: localDeployPath(path.join(__dirname,'target', 'deploy') , 'hydra') }
};

const validator = {
  verifyFees: false,
  commitment: 'confirmed',
  programs: [programs.metadata, programs.fanout]
};

module.exports = {
  programs,
  validator
};
