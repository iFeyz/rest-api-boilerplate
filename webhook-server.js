require('dotenv').config();
const express = require('express');
const { exec } = require('child_process');
const fs = require('fs');
const app = express();
const port = process.env.PORT || 4000;

// Middleware pour parser le JSON
app.use(express.json());

// Middleware d'authentification
function authenticate(req, res, next) {
  const authHeader = req.headers.authorization;
  if (!authHeader || !authHeader.startsWith('Bearer ')) {
    return res.status(401).json({ error: 'Unauthorized' });
  }
  
  const token = authHeader.split(' ')[1];
  if (token !== process.env.WEBHOOK_TOKEN) {
    return res.status(403).json({ error: 'Forbidden' });
  }
  
  next();
}

// Route de déploiement
app.post('/deploy', authenticate, (req, res) => {
  const { image, commit } = req.body;
  
  if (!image) {
    return res.status(400).json({ error: 'Image parameter is required' });
  }
  
  console.log(`[${new Date().toISOString()}] Deploying image: ${image} (commit: ${commit || 'unknown'})`);
  
  // Journaliser la demande de déploiement
  fs.appendFileSync('/home/votre-utilisateur/webhook/deploy.log', 
    `[${new Date().toISOString()}] Deploying ${image} (commit: ${commit || 'unknown'})\n`);
  
  // Exécuter le script de déploiement
  exec(`${process.env.DEPLOY_SCRIPT} "${image}" "${commit || ''}"`, (error, stdout, stderr) => {
    if (error) {
      console.error(`[${new Date().toISOString()}] Deployment error: ${error}`);
      fs.appendFileSync('/home/votre-utilisateur/webhook/deploy.log', 
        `[${new Date().toISOString()}] ERROR: ${stderr}\n`);
      return res.status(500).json({ error: 'Deployment failed', details: stderr });
    }
    
    console.log(`[${new Date().toISOString()}] Deployment output: ${stdout}`);
    fs.appendFileSync('/home/votre-utilisateur/webhook/deploy.log', 
      `[${new Date().toISOString()}] SUCCESS: ${stdout}\n`);
    res.json({ success: true, message: 'Deployment triggered' });
  });
});

// Route de vérification de santé
app.get('/health', (req, res) => {
  res.json({ status: 'ok', timestamp: new Date().toISOString() });
});

// Démarrer le serveur
app.listen(port, '0.0.0.0', () => {
  console.log(`[${new Date().toISOString()}] Webhook server listening at http://0.0.0.0:${port}`);
});