class Solution {
public:
    int maxArea(vector<int>& height) {
        int i = 0, j = height.size() - 1;
        int a, b;

        int maxArea = 0;

        while (i < j) {
            a = height[i];
            b = height[j];

            maxArea = max(maxArea, ((j - i) * min(a, b)));

            if (a <= b){
                i++;
            } else {
                j--;
            }
        }

        return maxArea;  
    }
};