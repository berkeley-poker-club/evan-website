OCF_USER="pokeratberkeley"
OCF_HOST="ssh.ocf.berkeley.edu"
REMOTE_PATH="/home/$OCF_USER/public_html"

echo "building site"
npm run build

echo "deploying to OCF as $OCF_USER"

rsync -avz --delete \
    --exclude='.DS_Store' \
    --exclude='*.map' \
    dist/ $OCF_USER@$OCF_HOST:~/public_html/

echo "deployment complete"
echo "site should be available at:"
echo "   https://poker.berkeley.edu (once virtual host is set up)"
echo "   https://www.ocf.berkeley.edu/~pokeratberkeley (default)"
